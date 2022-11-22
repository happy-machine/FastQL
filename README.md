# gqlwrapper

## TODO

* Add callbacks (class?) to Python and Rust to trigger model and send response
* Switch to async dependency so we can await on both sides
* Add dynamic definitions for juniper resolver
* Add inputs and outputs (params and fields API to Python)
* Add doc strings fields to Python API and corresponding Rust bindings 
* Deactivate rust and make all Python code still run transparently with GQL_WRAPPER_ACTIVE unset
* Convert resolver to subscription 
* Test sending a base64 encoded image 
* Add option to send path to image 
* Test with model
* Test as dependency
* Test with defined multiple build targets
* Documentation

Callback example:

function pythonCB(rustCB, params){
console.log("python callback running model with params:", params)
rustCB("response sent from python model")
}

function rustCB(callback){
  console.log("i am rust callback i am goung to return the response to the query")
}

console.log("in rust calling callback 1 to run model with params")
pythonCB(rustCB, {thing: "value"})

## useful links

https://stackoverflow.com/questions/41081240/idiomatic-callbacks-in-rust

https://alpha2phi.medium.com/serving-ml-model-using-graphql-subscription-7482d77ea061

https://stackoverflow.com/questions/57825509/is-this-the-idiomatic-way-to-share-a-closure-callback-among-threads-in-rust

https://graphql-rust.github.io/types/objects/complex_fields.html

https://docs.rs/juniper/latest/juniper/trait.GraphQLType.html

https://stackoverflow.com/questions/71357427/how-to-pass-a-rust-function-as-a-callback-to-python-using-pyo3