use crate::db_schema::videos::dsl::videos;
use crate::db_schema::videos::{description, title};
use crate::schemas::project_schemas::{PaginatedVideos, PaginatedVideosFilter, User, Video};
use crate::youtube::{Youtube, YoutubeError};
use crate::DbPool;
use actix_web::error;
use async_graphql::{Context, EmptySubscription, ErrorExtensions, FieldResult, Object, Schema};
use chrono::Utc;
use diesel::prelude::*;
use diesel::QueryDsl;
use std::fmt::{Display, Formatter};

pub struct Query;

impl Display for YoutubeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Internal server error")
    }
}

impl error::ResponseError for YoutubeError {}

#[Object]
impl Query {
    async fn profile(&self, ctx: &Context<'_>) -> FieldResult<User> {
        use crate::db_schema::users::dsl::*;
        let conn: &mut PgConnection = &mut ctx
            .data_unchecked::<DbPool>()
            .get()
            .expect("couldn't get db connection from pool");

        let user_id = 1;
        let user = users
            .filter(id.eq(user_id))
            .limit(1)
            .load::<User>(conn)
            .expect("Error loading user");

        Ok(user[0].clone())
    }

    async fn search_videos(
        &self,
        ctx: &Context<'_>,
        query: String,
        #[graphql(default = 20)] max_results: u32,
    ) -> FieldResult<Vec<Video>> {
        let conn: &mut PgConnection = &mut ctx
            .data_unchecked::<DbPool>()
            .get()
            .expect("Couldn't get db connection from pool");

        let mut results: Vec<Video> = videos
            .filter(
                title
                    .ilike(format!("%{}%", query.clone()))
                    .or(description.ilike(format!("%{}%", query.clone()))),
            )
            .limit(max_results as i64)
            .load::<Video>(conn)
            .map_err(|e| e.extend_with(|_, e| e.set("code", 500)))?;

        if results.len() == max_results as usize {
            return Ok(results);
        }

        let youtube: &Youtube = ctx.data_unchecked::<Youtube>();
        let mut youtube_results = youtube
            .query(query, 20)
            .await
            .map_err(|e| e.extend_with(|_, e| e.set("code", 500)))?;

        let b = &mut youtube_results;
        results.append(b);
        results.truncate(max_results as usize);

        Ok(results)
    }

    async fn videos(
        &self,
        ctx: &Context<'_>,
        filter: PaginatedVideosFilter,
    ) -> FieldResult<PaginatedVideos> {
        let conn: &mut PgConnection = &mut ctx
            .data_unchecked::<DbPool>()
            .get()
            .expect("Couldn't get db connection from pool");

        Video::paginated(filter, conn).map_err(|e| e.extend_with(|_, e| e.set("code", 500)))
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
            published_at: Utc::now(),
            rating: Some(0.0),
        })
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
