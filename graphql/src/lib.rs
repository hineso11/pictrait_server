mod profile;

use crate::profile::Profile;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, ID};
use std::fmt::Formatter;

pub struct Query;

#[derive(Debug, Clone)]
pub struct GraphQLError {
    message: String,
}

impl std::fmt::Display for GraphQLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn internal() -> GraphQLError {
    return GraphQLError {
        message: String::from("An unknown error occurred. Please try again."),
    };
}

pub fn not_found() -> GraphQLError {
    return GraphQLError {
        message: String::from("Could not find the requested resource."),
    };
}

pub fn invalid_argument(message: String) -> GraphQLError {
    return GraphQLError { message };
}

type Result<T> = std::result::Result<T, GraphQLError>;

#[Object]
impl Query {
    async fn find_user<'_ctx>(&self, _ctx: &Context<'_ctx>, _user_id: ID) -> Result<Profile> {
        Ok(Profile::new(
            3,
            String::from("Oliver"),
            String::from("Hines"),
        ))
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn new_schema() -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}
