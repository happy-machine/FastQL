use juniper::{EmptySubscription, FieldResult, RootNode};

#[derive(GraphQLEnum)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    pub id: String,
    pub name: String,
    pub appears_in: Vec<i32>,
    pub home_planet: String,
    pub age: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<i32>,
    home_planet: String,
    age: i32,
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
            age: new_human.age,
        })
    }
}
