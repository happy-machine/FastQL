#[macro_use]
extern crate lazy_static;
use actix_cors::Cors;
use std::sync::{Mutex};
use actix_web_lab::respond::Html;
use actix_web::{
    get,
    middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::http::{playground::playground_source, GraphQLRequest};
use juniper::{EmptySubscription, FieldResult, RootNode, EmptyMutation};
use pyo3::prelude::*;
use std::thread;
use std::error::Error;
use std::{io, sync::Arc};
mod schema;
use crate::schema::{Model, Params};
pub struct QueryRoot;



fn wrapper() {
    let global_cb: Option<Arc<Mutex<Box<dyn FnMut()>>>> = None;
    type CB = Arc<Mutex<Box<dyn FnMut()>>>;
    fn use_cb(callback: CB){
        if let Ok(mut cb) = callback.try_lock(){
            cb();
        }
    }
    // let b: Box<dyn FnMut()> = Box::new(|| println!("callback"));
    // let callback = Arc::new(Mutex::new(b));
    if let Some(value) = global_cb {
        use_cb(value);
    }
    else {
        println!("x is not set");
    }

    #[juniper::graphql_object]
    impl QueryRoot {
        fn model<'mdl>(&self, _params: Params) -> FieldResult<Model> {
            thread::spawn(move || Python::with_gil(|py| -> Result<(),Box<dyn Error + Send + Sync>> {
            println!("HERE IN RUST FUNC");

            //PyResult<Py<PyAny>>
            // let python_module: Py<PyAny> = PyModule::import(py, "model")?
            //     .call_method0("runner")?
            //     .into();
            // python_module.call0(py)
            let python_module: Py<PyAny> = PyModule::import(py, "model")?
                .call_method0("runner")?
                .into();
            // let python_module: Py<PyAny> = PyModule::import(py, "model")?
            // .getattr("runner")?
            // .call0()?
            // .into();
            let result = python_module.call0(py);
            //         let python_module: Py<PyAny> = PyModule::import(py, "model")?    
            //             .getattr("set_response_callback")?
            //             .call1((callback.into_py(py),"test".to_string()))?;
            println!("res: {:?}", result);
            //     let result = py.eval("[i * 10 for i in range(5)]", None, None).map_err(|e| {
            //     e.print_and_set_sys_last_vars(py);
            // })?;
            // let res: Vec<i64> = result.extract().unwrap();
            Ok(())
        }));
        //    let _res = thread_join_handle.join();
            // I WANT TO BE ABLE TO CALL THE CALLBACK HERE WITH MY GRAPHQLDATA!
            Ok(Model {
                model: _params.model.to_owned(),
                artifact_type: _params.artifact_type,
                artifact: _params.artifact.to_owned(),
                images: vec!["233242".to_string()],
                tokens: _params.tokens.to_owned(),
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

    #[get("/graphiql")]
    async fn graphql_playground() -> impl Responder {
        Html(playground_source("/graphql", None))
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
                //main();
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
                    // unsafe {
                    //     REQUEST_CALLBACK.lock().unwrap().push(callback);
                    // }
                    // send the callback back to python - this is the bit we want to happen on request
                    python_api
                        .getattr("set_response_callback")?
                        .call1((callback.into_py(py),"test".to_string()))?;
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
                m.add_function(wrap_pyfunction!(rust_register_callback, m)?)?;
                m.add_class::<Callback>()?;
                Ok(())
            }
            App::new()
                .app_data(Data::from(schema.clone()))
                .service(graphql)
                .service(graphql_playground)
                .wrap(Cors::permissive())
                // .wrap(middleware::Logger::default())
        })
        .workers(2)
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}