use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyTuple};
use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use std::thread;

use async_graphql::{
    dynamic::*,
    http::GraphiQLSource,
    EmptyMutation,
    EmptySubscription,
    Object,
    Value, ObjectType
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

async fn index(schema: web::Data<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    let inner = req.into_inner();
    schema.execute(inner).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000")
                .finish(),
        ))
}


#[actix_web::main]
pub async fn start_server(query: Object) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // let query = Object::new("Query")
    //     .field(Field::new("howdy", TypeRef::named_nn(TypeRef::STRING), |_| FieldFuture::new(async {
    //         Ok(Some(Value::from("partner")))
    //     })));

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

#[pyfunction]
fn py_start_server(fields: Vec<String>) -> PyResult<()> {
  println!("001 {:?}", fields);
  
  let mut query = Object::new("Query");

  let mut gqlFields: Vec<Field> = Vec::new();

  for field in fields {
    let gqlField = Field::new(field.to_string(), TypeRef::named_nn(TypeRef::INT), |_| FieldFuture::new(async{

      let out2: PyResult<i32> = Python::with_gil(|py| {

          // TEST
          let test = PyModule::import(py, "model2")?;
          let test_fn = test.getattr("run_model")?;
          let result: i32 = test_fn.call0()?.extract()?;
          println!("012 result: {}", result);
          // EO TEST
          Ok(result)
      });

      Ok(Some(Value::from(out2.unwrap())))
    }));

    query = query.field(gqlField);
  }
  thread::spawn(move || start_server(query));

  println!("002");
  Ok(())
}

#[pymodule]
#[pyo3(name = "GQLwrapper")]
fn st_df_2(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_start_server, m)?)?;
    // m.add_function(wrap_pyfunction!(init_graphql_server, m)?)?;

    Ok(())
}