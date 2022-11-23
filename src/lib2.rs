#[macro_use]
// extern crate lazy_static;
use actix_cors::Cors;
use std::sync::{Mutex};
use actix_web_lab::respond::Html;
use actix_web::{
    get,
    middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
// use juniper::http::{playground::playground_source, GraphQLRequest};
// use juniper::{EmptySubscription, FieldResult, RootNode, EmptyMutation};
use pyo3::prelude::*;
use std::thread;
use std::error::Error;
use std::{io, sync::Arc};

use async_graphql::{
    dynamic::*,
    http::GraphiQLSource,
    EmptyMutation,
    EmptySubscription,
    Object,
    Value, ObjectType
};

// mod schema;
// use crate::schema::{Model, Params};
pub struct QueryRoot;
use std::time::Duration;

extern crate zmq;

#[juniper::graphql_object]
impl QueryRoot {
    fn model<'mdl>(&self, _params: Params) -> FieldResult<Model> {
        let context = zmq::Context::new();
        let sender = context.socket(zmq::REQ).unwrap();
        let reciever = context.socket(zmq::REP).unwrap();
        assert!(sender.bind("tcp://*:5555").is_ok());
        sender.send(&_params.prompt.to_owned(), 0).unwrap();
        let mut msg = zmq::Message::new();            
        assert!(reciever.bind("tcp://*:5556").is_ok());
        loop {
            reciever.recv(&mut msg, 0).unwrap();
            println!("received: {}", msg.as_str().unwrap());
            break;
        }
        Ok(Model {
            model: _params.model.to_owned(),
            artifact_type: _params.artifact_type,
            artifact: _params.artifact.to_owned(),
            images: vec![msg.as_str().unwrap().to_string()],
            tokens: _params.tokens.to_owned(),
            prompt: _params.prompt.to_owned(),
        })
    }
}

// type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;
// fn create_schema() -> Schema {
//     Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
// }

// #[route("/graphql", method = "GET", method = "POST")]
// async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
//     let user = data.execute(&st, &()).await;
//     HttpResponse::Ok().json(user)
// }

// #[get("/graphiql")]
// async fn graphql_playground() -> impl Responder {
//     Html(playground_source("/graphql", None))
// }

// #[actix_web::main]
// pub async fn main() -> io::Result<()> {
//     std::env::set_var("RUST_LOG", "actix_web=info");
//     env_logger::init();

//     let schema = Arc::new(create_schema());
//     HttpServer::new(move || {
//         #[pyfunction]
//         fn init() -> PyResult<String> {
//             thread::spawn(move || main());
//             Ok("GQL server started...".to_string())
//         }
//         #[pymodule]
//         #[pyo3(name = "GQLwrapper")]
//         fn GQLwrapper(_py: Python, m: &PyModule) -> PyResult<()> {
//             m.add_function(wrap_pyfunction!(init, m)?)?;
//             Ok(())
//         }
//         App::new()
//             .app_data(Data::from(schema.clone()))
//             .service(graphql)
//             .service(graphql_playground)
//             .wrap(Cors::permissive())
//             // .wrap(middleware::Logger::default())
//     })
//     .workers(2)
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// NEW

#[actix_web::main]
pub async fn start_server() -> io::Result<()> {


    let query = Object::new("Query")
        .field(Field::new("howdy", TypeRef::named_nn(TypeRef::STRING), |_| FieldFuture::new(async {
            Ok(Some(Value::from("partner")))
        })));

    println!("001 {}", query.type_name());

    let schema = Schema::build(query.type_name(), None, None)
        .register(query)
		// .data(MyData::new())
        .finish();

    let schema2 = schema.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema2.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
      })
      .bind("127.0.0.1:8000")?
      .run()
      .await  
}