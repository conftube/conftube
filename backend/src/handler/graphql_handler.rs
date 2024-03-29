use crate::auth::UserInfo;
use crate::db_schema::users::id;
use crate::db_schema::videos::dsl::videos;
use crate::db_schema::videos::{description, title};
use crate::schemas::project_schemas::{
    AddVideoInput, PaginatedVideos, PaginatedVideosFilter, RateVideoInput, User, Video,
};
use crate::youtube::{Youtube, YoutubeError};
use crate::DbPool;
use actix_web::error;
use async_graphql::{
    Context, EmptySubscription, Error, ErrorExtensions, FieldResult, Object, Schema,
};
use diesel::prelude::*;
use diesel::{insert_into, QueryDsl};
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
        use crate::db_schema::*;

        let user_info = ctx.data_unchecked::<UserInfo>();
        let conn: &mut PgConnection = &mut ctx
            .data_unchecked::<DbPool>()
            .get()
            .expect("couldn't get db connection from pool");

        let user_id = user_info.user.id;
        let user = users::table
            .filter(id.eq(user_id))
            .limit(1)
            .get_result::<User>(conn)?;

        Ok(user)
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
    async fn add_video(&self, ctx: &Context<'_>, input: AddVideoInput) -> FieldResult<Video> {
        let video = match input.platform.as_str() {
            "youtube" => {
                let youtube: &Youtube = ctx.data_unchecked::<Youtube>();

                youtube
                    .find_by_id(input.id)
                    .await
                    .map_err(|e| e.extend_with(|_, e| e.set("code", 500)))
            }
            _ => Err(Error::new("Platform not found")),
        }?;

        let conn: &mut PgConnection = &mut ctx
            .data_unchecked::<DbPool>()
            .get()
            .expect("Couldn't get db connection from pool");

        insert_into(videos)
            .values(&video)
            .on_conflict_do_nothing()
            .execute(conn)?;

        Ok(video)
    }

    async fn rate_video(&self, ctx: &Context<'_>, input: RateVideoInput) -> FieldResult<Video> {
        use crate::db_schema::*;

        let conn: &mut PgConnection = &mut ctx
            .data_unchecked::<DbPool>()
            .get()
            .expect("Couldn't get db connection from pool");

        let user = &ctx.data_unchecked::<UserInfo>().user;

        let video = videos::table
            .filter(videos::id.eq(input.id))
            .first::<Video>(conn)?;

        let rated_video = video.rate(user, input.rating, conn)?;

        Ok(rated_video)
    }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
