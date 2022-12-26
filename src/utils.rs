use async_graphql::{
    dynamic::*,
};

pub fn type_factory<'a>(gql_type: &str) -> (async_graphql::dynamic::TypeRef, &'a str) {
let out = match gql_type {
    "String" => (TypeRef::named(TypeRef::STRING), "String"),
    "URL" => (TypeRef::named(TypeRef::STRING), "String"),
    "Int" => (TypeRef::named(TypeRef::INT), "Int"),
    "Boolean" => (TypeRef::named(TypeRef::BOOLEAN), "Boolean"),
    "Float" => (TypeRef::named(TypeRef::FLOAT), "Float"),
    "ID" => (TypeRef::named(TypeRef::ID), "ID"),
    "String!" => (TypeRef::named_nn(TypeRef::STRING), "String"),
    "URL!" => (TypeRef::named_nn(TypeRef::STRING), "String"),
    "Int!" => (TypeRef::named_nn(TypeRef::INT), "Int"),
    "Boolean!" => (TypeRef::named_nn(TypeRef::BOOLEAN), "Boolean"),
    "Float!" => (TypeRef::named_nn(TypeRef::FLOAT), "Float"),
    "ID!" => (TypeRef::named_nn(TypeRef::ID), "ID"),
    "[String]" => (TypeRef::named_list(TypeRef::STRING), "[String]"),
    "[URL]" => (TypeRef::named_list(TypeRef::STRING), "[String]"),
    "[Int]" => (TypeRef::named_list(TypeRef::INT), "[Int]"),
    "[Boolean]" => (TypeRef::named_list(TypeRef::BOOLEAN), "[Boolean]"),
    "[Float]" => (TypeRef::named_list(TypeRef::FLOAT), "[Float]"),
    "[ID]" => (TypeRef::named_list(TypeRef::ID), "[ID]"),
    "[String]!" => (TypeRef::named_list_nn(TypeRef::STRING), "[String]"),
    "[URL]!" => (TypeRef::named_list_nn(TypeRef::STRING), "[String]"),
    "[Int]!" => (TypeRef::named_list_nn(TypeRef::INT), "[Int]"),
    "[Boolean]!" => (TypeRef::named_list_nn(TypeRef::BOOLEAN), "[Boolean]"),
    "[Float]!" => (TypeRef::named_list_nn(TypeRef::FLOAT), "[Float]"),
    "[ID]!" => (TypeRef::named_list_nn(TypeRef::ID), "[ID]" ),
    _ => panic!("Type {:?} is not allowed", gql_type)
};
return out
}