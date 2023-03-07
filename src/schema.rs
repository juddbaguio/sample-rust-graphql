use async_graphql::{EmptySubscription, MergedObject, Schema};

use crate::{
    middleware,
    models::student::{StudentMutation, StudentQuery},
    services::student::StudentsCtx,
};

#[derive(MergedObject, Default)]
pub struct RootQuery(StudentQuery);

#[derive(MergedObject, Default)]
pub struct RootMutation(StudentMutation);

pub type StudentsSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

pub fn init_schema(ctx: StudentsCtx) -> StudentsSchema {
    Schema::build(
        RootQuery::default(),
        RootMutation::default(),
        EmptySubscription,
    )
    .extension(middleware::HeaderMiddleware)
    .extension(middleware::AuthMiddleware)
    .data(ctx)
    .finish()
}
