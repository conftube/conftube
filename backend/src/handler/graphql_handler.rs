use crate::schemas::project_schemas::{User, Video};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};
use diesel::QueryDsl;
use diesel::prelude::*;
use crate::{DbPool};

pub struct Query;

#[Object]
impl Query {
    async fn profile(&self, ctx: &Context<'_>) -> FieldResult<User> {
        use crate::db_schema::users::dsl::*;
        let conn: &mut PgConnection = &mut ctx.data_unchecked::<DbPool>().get().expect("couldn't get db connection from pool");

        let user_id = 1;
        let user = users
            .filter(id.eq(user_id))
            .limit(1)
            .load::<User>(conn)
            .expect("Error loading user");

        Ok(user[0].clone())
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn rate_video(&self, _ctx: &Context<'_>) -> FieldResult<Video> {
        Ok(Video {
            id: "".to_string(),
            platform: "".to_string(),
            title: "".to_string(),
            description: "".to_string(),
            thumbnail_url: "".to_string(),
            published_at: "".to_string(),
            rating: 0.0,
        })
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
