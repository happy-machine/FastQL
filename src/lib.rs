use pyo3::prelude::*;
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result, http::header};
use std::sync::{ Mutex };
use std::thread;
use actix_cors::Cors;
use async_graphql::{
    http::playground_source,
    http::GraphQLPlaygroundConfig,
    dynamic::*,
    Value,
};
use std::collections::HashMap;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::extensions::ApolloTracing;
use serde_json::json;
use tap::Pipe;

mod config;
mod types;
mod utils;

use config::constants::get_env;
use types::{StringOrStringVec, Context};
use utils::{type_factory};


async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
  let inner = req.into_inner();
  schema.execute(inner).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
  let env = get_env();
  Ok(HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(playground_source(GraphQLPlaygroundConfig::new(&format!("http://{}:{}", env.graphql_host, env.graphql_port))))
    )
}

#[actix_web::main]
pub async fn start_server(query: Object, model: Object) -> std::io::Result<()> {
  let env = get_env();
  if !env.rust_quiet {
    env_logger::init();
  }

  let zmq_context = zmq::Context::new();
  let context = Context {
      zmq_sender: Mutex::new(zmq_context.socket(zmq::REQ).unwrap()),
  };
  assert!(context.zmq_sender.lock().unwrap().bind(&format!("tcp://*:{}", env.zeromq_port)).is_ok());
  let schema = Schema::build(query.type_name(), None, None)
    .register(model)
    .register(query)
    .pipe(|e| if env.tracing{e.extension(ApolloTracing)} else {e})
    .data(context)
    .finish();

  let schema_temp = schema.unwrap();

  HttpServer::new(move || {
    let cors = if env.enable_cors {
      Cors::permissive()
    } else {
      Cors::default()
      .allowed_methods(vec!["GET", "POST"])
      .pipe(|e| if env.allowed_origin_header == "*".to_string(){e.send_wildcard()} else {e.allowed_origin(&env.allowed_origin_header)})
      .pipe(|e| if env.allow_authorization_header{e.allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])} else {e})
      .pipe(|e| if env.allow_content_type_header{e.allowed_header(header::CONTENT_TYPE)} else {e})
      .max_age(env.max_age_header)
    };

    App::new()
    .wrap(cors)
        .app_data(Data::new(schema_temp.clone()))
        .service(web::resource("/").guard(guard::Post()).to(index))
        .pipe(|e| if env.enable_graphiql { return e.service(web::resource("/graphql").guard(guard::Get()).to(index_graphiql))} else {return e})
  })
     .bind(env.graphql_endpoint)?
  .run()
  .await  
}

#[pyfunction]
fn init<'a>(
  query_name: String,
  params: HashMap<String, HashMap<String, String>>,
  fields: HashMap<String, HashMap<String, String>>
) -> PyResult<()> {
  let mut model = Object::new(&query_name);
  let mut modelField = Field::new(
    &query_name,
    TypeRef::named_nn(model.type_name()),
    |ctx| FieldFuture::new(async move{
      let selection_fields = &ctx.look_ahead().selection_fields();
      let mut params_hashmap: HashMap<String, String> = HashMap::new();
      for params in selection_fields[0].arguments().iter() {
        for inner_param in params {
          let inner_params: (async_graphql::Name, async_graphql::Value) = inner_param.clone();
          params_hashmap.insert(inner_params.0.to_string(), inner_params.clone().1.to_string().trim_matches('\"').to_string());
        }
      }
      let context = ctx.data::<Context>()?;
      let sender = context.zmq_sender.lock().unwrap();
      sender.send(&json!(params_hashmap).to_string(), 0).unwrap();
      let mut msg = zmq::Message::new();
      sender.recv(&mut msg, 0).unwrap();
      let deserialized: HashMap<String, StringOrStringVec> = serde_json::from_str(&msg.as_str().unwrap()).unwrap();
      let deserialized: HashMap<String, Vec<String>> = deserialized.into_iter().map(|(key, val)| match val {
          StringOrStringVec::String(s) => (key, vec![s]),
          StringOrStringVec::VecString(v) => (key, v),
          StringOrStringVec::Float(s) => (key, vec![s.to_string()]),
          StringOrStringVec::VecFloat(v) => (key, v.iter().map(|w| w.to_string()).collect()),
          StringOrStringVec::Boolean(s) => (key, vec![s.to_string()]),
          StringOrStringVec::VecBoolean(v) => (key, v.iter().map(|w| w.to_string()).collect()),
          StringOrStringVec::ID(s) => (key, vec![s.to_string()]),
          StringOrStringVec::VecID(v) => (key, v.iter().map(|w| w.to_string()).collect()),
          StringOrStringVec::Int(s) => (key, vec![s.to_string()]),
          StringOrStringVec::VecInt(v) => (key, v.iter().map(|w| w.to_string()).collect()),
      }).collect();
      Ok(Some(FieldValue::owned_any(deserialized)))
    })
  );

  for (key, val) in params.iter() {
    modelField = match val.get("description") {
      None => modelField.argument(InputValue::new(
        key,
        type_factory(val.get("type").unwrap().as_str()).0,
      )),
      Some(value) => modelField.argument(InputValue::new(
        key,
        type_factory(val.get("type").unwrap().as_str()).0,
      ).description(value))
    };
  }
  let query = Object::new("Query").field(
    modelField
  );

  for (key, val) in fields.iter() {
    let type_factory_result = type_factory(val.get("type").unwrap().as_str());
    let field = Field::new(
      key.to_string(),
      type_factory_result.0,
      move |ctx:ResolverContext| FieldFuture::new(async move {
          let field_hashmap = ctx.parent_value.try_downcast_ref::<HashMap<String, Vec<String>>>()?;
          let value = field_hashmap.get(ctx.field().name());
          let out = value.unwrap().clone();
          let result = match type_factory_result.1 {
            "String" => {
              let string_val = out[0].clone();
              Ok(Some(Value::from(string_val.to_string())))
            },
            "Int" => {
              let string_val = out[0].clone();
              Ok(Some(Value::from(string_val.to_string().parse::<u64>().unwrap())))
            },
            "Boolean" => {
              let string_val = out[0].clone();
              Ok(Some(Value::from(string_val.to_string().parse::<bool>().unwrap())))
            },
            "Float" => {
              let string_val = out[0].clone();
              Ok(Some(Value::from(string_val.to_string().parse::<f32>().unwrap())))
            },
            "ID" => {
              let string_val = out[0].clone();
              Ok(Some(Value::from(string_val.to_string())))
            },
            "[String]" => Ok(Some(Value::List(value.unwrap().clone().into_iter().map(Value::from).collect()))),
            "[Int]" => Ok(Some(Value::List(out.into_iter().map(|x| x.parse::<u64>().unwrap()).map(Value::from).collect()))),
            "[Boolean]" => Ok(Some(Value::List(out.into_iter().map(|x| x.parse::<bool>().unwrap()).map(Value::from).collect()))),
            "[Float]" => Ok(Some(Value::List(out.into_iter().map(|x| x.parse::<f32>().unwrap()).map(Value::from).collect()))),
            "[ID]" => Ok(Some(Value::List(out.into_iter().map(Value::from).collect()))),
            _ => Ok(Some(Value::List(out.into_iter().map(Value::from).collect()))),
          };
          return result.clone();
        })
    );

    model = match val.get("description") {
      None => model.field(field),
      Some(value) => model.field(field.description(value))
    };
  }
  thread::spawn(move || start_server(query, model));
  Ok(())
}

#[pymodule]
#[pyo3(name = "fastqlapi")]
fn st_df_2(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(init, m)?)?;
  Ok(())
}