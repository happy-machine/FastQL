use pyo3::prelude::*;
use std::cell::Cell;
use std::{io, sync::Arc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
use actix_cors::Cors;
use actix_web::{get, middleware, route, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

static GLOBAL_DATA: Mutex<Vec<i32>> = Mutex::new(Vec::new());

mod schema;
use crate::schema::{create_schema, Schema};
/// simple handle
fn wrapper(){
    // struct Memo {
    //     value: String
    // }

    // impl Memo {
    //     fn get(self) -> String {
    //         self.value
    //     }
    //     fn set(self, new: String) -> () {
    //         self.value = new;
    //     }
    // }
    // let shared = Memo {
    //     value: "".to_string()
    // };
    // static mut shared:String;
 
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    // match serde_json::to_string(&user.serialize()) {
    //     Ok(result) => query = result,
    //     _ => {
    //         //
    //     }
    // }
    // println!("STUFFFFFFFF {:?}", user.get_field_value("age").unwrap().as_string_value().unwrap());
    GLOBAL_DATA.lock().unwrap().push(42);
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let schema = Arc::new(create_schema());
    // Create some global state prior to building the server
    #[allow(clippy::mutex_atomic)] // it's intentional.
    let counter1 = web::Data::new(Mutex::new(0usize));
    let counter3 = web::Data::new(AtomicUsize::new(0usize));

    // move is necessary to give closure below ownership of counter1
    HttpServer::new(move || {
        // Create some thread-local state
        let counter2 = Cell::new(0u32);

        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            // the graphiql UI requires CORS to be enabled
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            // .app_data(counter1.clone()) // add shared state
            // .app_data(counter3.clone()) // add shared state
            // .data(counter2) // add thread-local state
            // enable logger
            // .wrap(middleware::Logger::default())
            // // register simple handler
            // .service(web::resource("/").to(index))
    })
    .workers(2)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

    #[pyfunction]
    fn init(a: usize) -> PyResult<String> {
        thread::spawn(move || main());
        Ok((a * 2).to_string())
    }

    #[pyfunction]
    fn fetch() -> PyResult<i32> {
        Ok(GLOBAL_DATA.lock().unwrap()[0])
    }

    #[pymodule]
    fn gqltest(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(init, m)?)?;
        m.add_function(wrap_pyfunction!(fetch, m)?)?;
        Ok(())
    }
}