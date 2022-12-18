use serde::{Serialize, Deserialize};
use std::sync::{ Mutex };

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStringVec {
    String(String),
    VecString(Vec<String>),
    Float(f32),
    VecFloat(Vec<f32>),
    Boolean(bool),
    VecBoolean(Vec<bool>),
    ID(String),
    VecID(Vec<String>),
    Int(u64),
    VecInt(Vec<u64>)
}

pub struct Context {
    pub zmq_sender: Mutex<zmq::Socket>,
}
