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

use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_graphql::extensions::ApolloTracing;

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
    zmqReciever: Mutex<zmq::Socket>
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
        zmqSender: Mutex::new(zmqContext.socket(zmq::PUSH).unwrap()),
        zmqReciever: Mutex::new(zmqContext.socket(zmq::PULL).unwrap())
    };
    assert!(context.zmqSender.lock().unwrap().bind("tcp://*:5555").is_ok());
    assert!(context.zmqReciever.lock().unwrap().bind("tcp://*:5556").is_ok());

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
          .wrap(cors)
          .app_data(Data::new(schema2.clone()))
          .service(web::resource("/").guard(guard::Post()).to(index))
          .service(web::resource("/graphql").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await  
}

#[pyfunction]
fn init<'a>(fields: Vec<String>) -> PyResult<()> {
  println!("001 {:?}", fields);
  
  /*
    query {
        Model {
            images,
            tokens
        }
    }
   */
  let mut model = Object::new("Model");

  pub struct Model {}

  let mut query = Object::new("Query").field(
    Field::new(
        "Model",
        TypeRef::named_nn(model.type_name()),
        |_| FieldFuture::new(async{
            Ok(Some(()))
        })
    )
  );

  let mut gqlFields: Vec<Field> = Vec::new();

  for field in fields {
    let gqlField = Field::new(
        field.to_string(),
        TypeRef::named_nn(TypeRef::STRING),
        |ctx:ResolverContext| FieldFuture::new(async move {

            let context = ctx.data::<Context>()?;

            let nameField = match ctx.ctx.path_node.unwrap().segment {
                QueryPathSegment::Name(value) => value,
                _ => ""
            };

            let sender = context.zmqSender.lock().unwrap();
            sender.send(nameField, 0).unwrap();

            let reciever = context.zmqReciever.lock().unwrap();
            let mut msg = zmq::Message::new();
            loop {
                reciever.recv(&mut msg, 0).unwrap();
                break;
            }            
        
            Ok(Some(Value::from(msg.as_str().unwrap())))
            // Ok(Some(Value::from("test".to_string())))
        })
    );

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