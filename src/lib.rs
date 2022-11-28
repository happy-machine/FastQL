use pyo3::prelude::*;
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use core::panic;
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


async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    let inner = req.into_inner();
    schema.execute(inner).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("http://localhost:8000"))
        ))
}

struct Context {
    zmqSender: Mutex<zmq::Socket>,
}

#[actix_web::main]
pub async fn start_server(query: Object, model: Object) -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    let zmq_context = zmq::Context::new();
    let context = Context {
        zmqSender: Mutex::new(zmq_context.socket(zmq::REQ).unwrap()),
    };
    assert!(context.zmqSender.lock().unwrap().bind("tcp://*:5555").is_ok());

    let schema = Schema::build(query.type_name(), None, None)
        .register(model)
        .register(query)
        .extension(ApolloTracing)
        .data(context)
        .finish();

    let schema_temp = schema.unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive();
        // .allowed_origin("*")
        // .allowed_methods(vec!["GET", "POST"])
        // .allowed_header(http::header::CONTENT_TYPE)
        // .allowed_header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
        // .allowed_header(http::header::ACCESS_CONTROL_ALLOW_HEADERS)
        // .allowed_header(vec![http::header::AUTHORIZATION, http::header::ACCEPT]);
      App::new()
      .wrap(cors)
          .app_data(Data::new(schema_temp.clone()))
          .service(web::resource("/").guard(guard::Post()).to(index))
          .service(web::resource("/graphql").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await  
}

#[pyfunction]
fn init<'a>(
    params: HashMap<String, HashMap<String, String>>,
    fields: HashMap<String, HashMap<String, String>>
) -> PyResult<()> {
  let mut model = Object::new("Model");
  let mut modelField = Field::new(
    "Model",
    TypeRef::named_nn(model.type_name()),
    |ctx| FieldFuture::new(async move{
        let selection_fields = &ctx.look_ahead().selection_fields();
        let mut params_hashmap: HashMap<String, String> = HashMap::new();
        for params in selection_fields[0].arguments().iter() {
          for inner_param in params {
            let inner_params: (async_graphql::Name, async_graphql::Value) = inner_param.clone();
            params_hashmap.insert(inner_params.0.to_string(), inner_params.1.to_string().trim_matches('\"').to_string());
          }
        }
        let context = ctx.data::<Context>()?;
        let sender = context.zmqSender.lock().unwrap();
        sender.send(&json!(params_hashmap).to_string(), 0).unwrap();
        let mut msg = zmq::Message::new();
        sender.recv(&mut msg, 0).unwrap();
        let deserialized: HashMap<String, String> = serde_json::from_str(&msg.as_str().unwrap()).unwrap();
        Ok(Some(FieldValue::owned_any(deserialized)))
    })
  );

  for (key, val) in params.iter() {
    let type_factory = match val.get("type").unwrap().as_str() {
      "String" => TypeRef::STRING,
      "Int" => TypeRef::INT,
      "Boolean" => TypeRef::BOOLEAN,
      "Float" => TypeRef::FLOAT,
      _ => panic!("Type {:?} is not allowed", val.get("type").unwrap().as_str())
    };
    modelField = modelField.argument(InputValue::new(
      key,
      TypeRef::named(type_factory)
    ).description(val.get("description").unwrap_or(&"No docs yet!".to_string())));
  }

  let query = Object::new("Query").field(
    modelField
  );

  for (key, val) in fields.iter() {
    let field = Field::new(
        key.to_string(),
        TypeRef::named_nn(TypeRef::STRING),
        |ctx:ResolverContext| FieldFuture::new(async move {
            let field_hashmap = ctx.parent_value.try_downcast_ref::<HashMap<String, String>>()?;
            let value = field_hashmap.get(ctx.field().name());
            let out = value.unwrap();
            Ok(Some(Value::from(out.to_string())))
        })
    ).description(val.get("description").unwrap_or(&"No docs yet!".to_string()));
    model = model.field(field);
  }
  thread::spawn(move || start_server(query, model));
  Ok(())
}

#[pymodule]
#[pyo3(name = "fastql")]
fn st_df_2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    Ok(())
}