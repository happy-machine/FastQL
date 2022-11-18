use juniper::{EmptySubscription, FieldResult, RootNode};

#[derive(GraphQLEnum)]
pub enum ArtifactType {
  ARTIFACT_NONE,
  ARTIFACT_IMAGE,
  ARTIFACT_VIDEO,
  ARTIFACT_TEXT,
  ARTIFACT_TOKENS,
  ARTIFACT_EMBEDDING,
  ARTIFACT_CLASSIFICATIONS,
  ARTIFACT_MASK,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A stable diffusion model inference")]
pub struct Model {
    pub prompt: String,
    pub artifact: String,
    pub artifact_type: ArtifactType,
    pub model: String,
    pub tokens: Vec<String>,
    pub images: Vec<String>,
}
impl juniper::Context for Model {}

#[derive(GraphQLInputObject)]
#[graphql(description = "Optional params for inference")]
pub struct Params {
    pub prompt: String,
    pub artifact: String,
    pub artifact_type: ArtifactType,
    pub model: String,
    pub tokens:  Vec<String>,
}

pub struct MutationRoot;

#[juniper::graphql_object(context = Model)]
impl MutationRoot {
    fn create_model<'mdl>(&self, context: &'mdl Model,_params: Params) -> FieldResult<Model> {
        Ok(Model {
            model: "1234".to_owned(),
            images: vec!["WRET£34t3".to_string(), "WREWRTY£34t3".to_string()],
            artifact: params.artifact,
            artifact_type: params.artifact_type,
            tokens: params.tokens,
            prompt: params.prompt,
        })
    }
}