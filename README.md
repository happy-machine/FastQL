# gqlwrapper

## TODO

Add callbacks (class?) to Python and Rust to trigger model and send response
Add dynamic definitions for juniper resolver
Add inputs and outputs (params and fields API to Python)
Add doc strings fields to Python API and corresponding Rust bindings 
Deactivate rust and make all Python code still run transparently with GQL_WRAPPER_ACTIVE unset
Convert resolver to subscription 
Test sending a base64 encoded image 
Add option to send path to image 
Test with model
Test as dependency
Test with defined multiple build targets
Documentation

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

