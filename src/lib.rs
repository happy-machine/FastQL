#[macro_use]
extern crate lazy_static;
use actix_cors::Cors;
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{playground::playground_source, graphiql::graphiql_source, GraphQLRequest};
use juniper::{EmptySubscription, FieldResult, RootNode};
use pyo3::prelude::*;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
use std::{io, sync::Arc};
use std::collections::HashMap;
static GLOBAL_DATA: Mutex<Vec<String>> = Mutex::new(Vec::new());
// lazy_static! {
static REQUEST_CALLBACK: Mutex<Vec<Box<Callback>>> = Mutex::new(Vec::new());
// }

// lazy_static! {
//     static REQUEST_CALLBACK: Mutex<HashMap<u8, Box<dyn Fn(&str) + Send>>> = Mutex::new(HashMap::new());
// }
type Callback = Mutex<dyn 'static + FnMut(&PyAny) + Send + Sync>;
// static GLOBAL_CALLBACK: Callback = Arc::new(Mutex::new(|_,_| {}));
// struct LibThreaded {
//     something_threaded: String,
//     callback: Callback,
// }
mod schema;
use crate::schema::{ArtifactType, Model, MutationRoot, Params};

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn model(_params: Params) -> FieldResult<Model> {
        Ok(Model {
            model: _params.model.to_owned(),
            artifact_type: _params.artifact_type,
            artifact: _params.artifact.to_owned(),
            images: GLOBAL_DATA.lock().unwrap().to_owned(),
            tokens: _params.tokens.to_owned(),
            prompt: _params.prompt.to_owned(),
        })
    }
}

type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

fn wrapper() {

    #[get("/graphiql")]
    async fn graphql_playground() -> impl Responder {
        Html(playground_source("/graphql", None))
    }

    /// GraphQL endpoint
    #[route("/graphql", method = "GET", method = "POST")]
    async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
        let user = data.execute(&st, &()).await;
        Python::with_gil(|py| {
            let callback = Box::new(Callback {
                callback_function: Box::new(move |python_api| {
                    rust_callback(python_api, "stuff".to_string())
                }),
            });
            // let mut cbs = REQUEST_CALLBACK.try_lock().unwrap();
            // let cb = cbs.get(0).clone();
            //   python_api
            //     .getattr("set_response_callback")?
            //     .call1((cb.into_py(cb),))?;
            fn test() {

            }
            unsafe {
                let cb = &mut REQUEST_CALLBACK.lock().unwrap()[0];
                cb();
            }
            // let callback = Box::new(Callback {
            //     callback_function: Box::new(move |python_api| {
            //         rust_callback(python_api, message.clone())
            //     }),
            // });
            // python_api
            //     .getattr("set_response_callback")?
            //     .call1((callback.into_py(py),))?;
            //  Ok::<(), E>(())
        });
        HttpResponse::Ok().json(user)
    }

    #[actix_web::main]
    async fn main() -> io::Result<()> {
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();
        let schema = Arc::new(create_schema());

        // #[allow(clippy::mutex_atomic)]
        // let counter1 = web::Data::new(Mutex::new(0usize));
        // let counter3 = web::Data::new(AtomicUsize::new(0usize));

        // move is necessary to give closure below ownership of counter1
        HttpServer::new(move || {
            // let counter2 = Cell::new(0u32);

            App::new()
                .app_data(Data::from(schema.clone()))
                .service(graphql)
                .service(graphql_playground)
                .wrap(Cors::permissive())
                .wrap(middleware::Logger::default())
        })
        .workers(2)
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }

    #[pyfunction]
    fn init() -> PyResult<String> {
        thread::spawn(move || main());
        Ok("GQLwrapper initialised...".to_string())
    }
    #[pyfunction]
    fn set_params(value: i32) -> () {
        // do some dynamic crazy.
    }
    #[pyfunction]
    fn set_fields(value: String) -> () {
        GLOBAL_DATA.lock().unwrap().push(value);
    }
    // #[pyfunction]
    // fn fields(cb:  Box<dyn Fn(String)>) -> PyResult<String> {
    //     GLOBAL_CALLBACK.lock().unwrap() = cb;
    // }

    #[pyclass]
    struct Callback {
        #[allow(dead_code)] // callback_function is called from Python
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
        println!("This is rust_register_callback");
        let message: String = "a captured variable".to_string();
        Python::with_gil(|py| {
            // set up the rust callback and load it with rust callback for response
            let callback = Box::new(Callback {
                callback_function: Box::new(move |python_api| {
                    rust_callback(python_api, message.clone())
                }),
            });
            // let mut cbs  = REQUEST_CALLBACK.try_lock().unwrap();
            // cbs.insert(0, callback);
            unsafe {
                REQUEST_CALLBACK.lock().unwrap().push(callback);
            }
            // send the callback back to python - this is the bit we want to happen on request
            // python_api
            //     .getattr("set_response_callback")?
            //     .call1((callback.into_py(py),))?;
            Ok(())
        })
    }

    #[pyfunction]
    fn rust_callback(python_api: &PyAny, message: String) -> PyResult<()> {
        // This will be the callback that returns the response
        println!("This is rust_callback");
        println!("Message = {}", message);
        python_api.getattr("some_operation")?.call0()?;
        Ok(())
    }


    #[pymodule]
    #[pyo3(name = "GQLwrapper")]
    fn GQLwrapper(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(init, m)?)?;
        m.add_function(wrap_pyfunction!(set_fields, m)?)?;
        m.add_function(wrap_pyfunction!(rust_register_callback, m)?)?;
        m.add_function(wrap_pyfunction!(rust_callback, m)?)?;
        m.add_class::<Callback>()?;
        Ok(())
    }
}
