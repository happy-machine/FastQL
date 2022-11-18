#[macro_use]
extern crate lazy_static;
use actix_cors::Cors;
use actix_web::{
    middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::http::{GraphQLRequest};
use juniper::{EmptySubscription, FieldResult, RootNode, EmptyMutation};
use pyo3::prelude::*;
use std::thread;
use std::{io, sync::Arc};
mod schema;
use crate::schema::{Model, Params};
pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn model<'mdl>(&self, _params: Params) -> FieldResult<Model> {
        // I WANT TO BE ABLE TO CALL THE CALLBACK HERE WITH MY GRAPHQLDATA!
        Ok(Model {
            prompt: _params.prompt.to_owned(),
        })
    }
}

type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;
fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let schema = Arc::new(create_schema());
    HttpServer::new(move || {
        #[pyfunction]
        fn init() -> PyResult<String> {
            thread::spawn(move || main());
            Ok("GQL server started...".to_string())
        }

        #[pyclass]
        struct Callback {
            #[allow(dead_code)]
            callback_function: Box<dyn Fn(&PyAny) -> PyResult<()> + Send>,
        }

        #[pymethods]
        impl Callback {
            fn __call__(&self, python_api: &PyAny) -> PyResult<()> {
                (self.callback_function)(python_api)
            }
        }

        #[pyfunction]
        fn rust_register_callback(python_api: &PyAny) -> PyResult<()> {
            // this will be the callback that sends the request and the response callback
            Python::with_gil(|py| {
                // THIS IS THE CALLBACK I WANT TO BE ABLE TO CALL FROM THE RESOLVER
                let callback = Box::new(Callback {
                    callback_function: Box::new(move |python_api| {
                        rust_callback(python_api, "THIS IS WHERE I WANT TO PUT MY CAPTURED DATA FROM RESOLVER".to_string())
                    }),
                });
                println!("in rust register callback");
                python_api
                .getattr("set_response_callback")?
                .call1((callback.into_py(py),))?;
                Ok(())
            })
        }

        #[pyfunction]
        fn rust_callback(python_api: &PyAny, message: String) -> PyResult<()> {
            // This will ultimately be the callback that returns the response
            println!("This is rust_callback");
            println!("Message = {}", message);
            python_api.getattr("some_operation")?.call0()?;
            Ok(())
        }

        #[pymodule]
        #[pyo3(name = "GQLwrapper")]
        fn GQLwrapper(_py: Python, m: &PyModule) -> PyResult<()> {
            m.add_function(wrap_pyfunction!(init, m)?)?;
            m.add_function(wrap_pyfunction!(rust_register_callback, m)?)?;
            m.add_function(wrap_pyfunction!(rust_callback, m)?)?;
            m.add_class::<Callback>()?;
            Ok(())
        }
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

