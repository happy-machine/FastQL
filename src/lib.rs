use pyo3::ffi::PyRun_InteractiveLoop;
// use actix_web_lab::__reexports::tokio::sync::RwLock;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyTuple};
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result, http};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use actix_cors::Cors;
use async_graphql::{
    http::playground_source,
    http::GraphQLPlaygroundConfig,
    dynamic::*,
    http::GraphiQLSource,
    EmptyMutation,
    EmptySubscription,
    Object,
    Value, ObjectType,
    QueryPathSegment
};
use actix_http::{
    body::{BoxBody, EitherBody, MessageBody},
    header::HeaderMap,header::HeaderValue,
    Extensions, Response, ResponseHead, StatusCode,
};
use std::collections::HashMap;

use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::extensions::ApolloTracing;

use serde::{Serialize, Deserialize};

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    let inner = req.into_inner();
    schema.execute(inner).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        // .append_header((HeaderValue::from_static("access-control-allow-origin"), HeaderValue::from_static("*")))
        // .append_header((HeaderValue::from_static("access-control-allow-credentials"), HeaderValue::from_static("true")))
        .body(playground_source(GraphQLPlaygroundConfig::new("http://localhost:8000"))
        ))
}

struct Context {
    zmqSender: Mutex<zmq::Socket>,
}

#[actix_web::main]
pub async fn start_server(query: Object, model: Object) -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    // let query = Object::new("Query")
    //     .field(Field::new("howdy", TypeRef::named_nn(TypeRef::STRING), |_| FieldFuture::new(async {
    //         Ok(Some(Value::from("partner")))
    //     })));

    // println!("001 {}", query.type_name());

    let zmqContext = zmq::Context::new();
    let context = Context {
        zmqSender: Mutex::new(zmqContext.socket(zmq::REQ).unwrap()),
    };
    assert!(context.zmqSender.lock().unwrap().bind("tcp://*:5555").is_ok());

    let schema = Schema::build(query.type_name(), None, None)
        .register(model)
        .register(query)
        .extension(ApolloTracing)
		// .data(Context::init())
        .data(context)
        .finish();

    let schema2 = schema.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin("*")
        // .allowed_origin_fn(|origin, _req_head| {
        //     origin.as_bytes().ends_with(b".rust-lang.org")
        // })
        .allowed_methods(vec!["GET", "POST"])
        // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE);
        // .max_age(3600);
      App::new()
        //   .wrap(cors)
          .app_data(Data::new(schema2.clone()))
          .service(web::resource("/").guard(guard::Post()).to(index))
          .service(web::resource("/graphql").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await  
}

// struct Inputs {
//     fields: HashMap<String, (String, String)>,
//     params: HashMap<String, (String, String)>,
// }

#[pyfunction]
fn init<'a>(
    params: HashMap<String, HashMap<String, String>>,
    fields: HashMap<String, (String, String)>
) -> PyResult<()> {

  /*
    query {
        Model {
            images,
            tokens
        }
    }
   */
   // { param: { type: , description: }} = a
   //  { a, b, c, d} where a b c d are params 
   // ordered dict .. enforce the order of the dict so we know that a is 0, b is 1
   // on the rust side when we pull out of the hashmap the ordering
   // iterate the params we get from the request and for each param - what is the index of the param based on the order it was recieved in
   // send string messages back in the same order, python uses the ordering to map the message from zeromq to match the object
   // a b c d
   // c b a d
   // 2 1 0 3
   // 'string_a|string_b|string_c|string_d'
   // message 1 = a, message 2 = b
   // for each key on the ordered dict set the value to be the string.split(index of key)
  let mut model = Object::new("Model");

  pub struct Model {}

  let mut modelField = Field::new(
    "Model",
    TypeRef::named_nn(model.type_name()),
    |ctx| FieldFuture::new(async move{

        let lookAhead = &ctx.look_ahead();
        let selectionFields = lookAhead.selection_fields();

        let model = selectionFields[0];
        let mut modelString: Vec<String> = Vec::with_capacity(4);
        for selection in model.selection_set().into_iter() {
            modelString.push(selection.name().to_string());
        }
        let stringToSend = modelString.join(", ");

        let context = ctx.data::<Context>()?;
        let sender = context.zmqSender.lock().unwrap();
        sender.send(&stringToSend, 0).unwrap();

        let mut msg = zmq::Message::new();
        sender.recv(&mut msg, 0).unwrap();
        let deserialized: HashMap<String, String> = serde_json::from_str(&msg.as_str().unwrap()).unwrap();
        
        let mut testResponse: HashMap<String, String> = HashMap::new();
        testResponse.insert("images".to_string(), "image 1".to_string());
        testResponse.insert("tokens".to_string(), "tokens 1".to_string());
        

        Ok(Some(FieldValue::owned_any(deserialized)))
    })
  );

  for (index, (key, val)) in params.iter().enumerate() {

    let theType = match val.get("type").unwrap().as_str() {
      "String" => TypeRef::STRING,
      "Int" => TypeRef::INT,
      "Boolean" => TypeRef::BOOLEAN,
      _ => TypeRef::STRING
    };

    // let my_input = InputObject::new("MyInput")
    //   .oneof()
    //   .field(InputValue::new(key, TypeRef::named_nn(theType)))
    //   .description(val.get("description").unwrap_or(&"".to_string()));

    // let argument = modelField.argument(InputValue::new("input", TypeRef::named_nn(my_input.type_name())));

    modelField = modelField.argument(InputValue::new(
      key,
      TypeRef::named(theType)
    ).description(val.get("description").unwrap_or(&"".to_string())));

    // let my_input = my_input.field(InputValue::new(key.to_string(), TypeRef::String.non_null()));
  }

  let mut query = Object::new("Query").field(
    modelField
  );

  let mut gqlFields: Vec<Field> = Vec::new();

  // Start server defining fields and params
  // Get request
  // Send message over zeromq with serialized params
  // wait for zero message from python with the field contents
  // GraphQL response with the fields from python
  
 // 
  for (index, (key, val)) in fields.iter().enumerate() {
    let gqlField = Field::new(
        key.to_string(),
        TypeRef::named_nn(TypeRef::STRING),
        |ctx:ResolverContext| FieldFuture::new(async move {

            let theHashMap = ctx.parent_value.try_downcast_ref::<HashMap<String, String>>()?;
            let value = theHashMap.get(ctx.field().name());

            let out = value.unwrap();
            Ok(Some(Value::from(out.to_string())))
        })
    ).description(val.1.to_string());

    model = model.field(gqlField);
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