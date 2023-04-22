use crate::db_schema::ratings::{user_id, video_id};
use crate::db_schema::{ratings, users, videos};
use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use diesel::dsl::{avg, count};
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    AsChangeset,
    Serialize,
    Deserialize,
    SimpleObject,
    Queryable,
    Selectable,
    Insertable,
)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: Option<i32>,
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub picture: String,
}

impl NewUser {
    pub fn register(&self, conn: &mut PgConnection) -> Result<User, Error> {
        use crate::db_schema::*;

        self.insert_into(users::table)
            .on_conflict(users::email)
            .do_update()
            .set(self)
            .get_result::<User>(conn)
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    SimpleObject,
    Queryable,
    Identifiable,
    Selectable,
    Insertable,
)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub family_name: String,
    pub given_name: String,
    pub picture: String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    SimpleObject,
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
)]
#[diesel(table_name = videos)]
pub struct Video {
    pub id: String,
    pub platform: String,
    pub title: String,
    pub description: String,
    pub published_at: DateTime<Utc>,
    pub thumbnail_url: String,
    pub rating: Option<f64>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    SimpleObject,
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    Associations,
)]
#[diesel(belongs_to(Video))]
#[diesel(table_name = ratings)]
pub struct Rating {
    pub id: Option<i32>,
    pub video_id: String,
    pub user_id: i32,
    pub rating: f64,
}

impl Video {
    pub fn paginated(
        filter: PaginatedVideosFilter,
        conn: &mut PgConnection,
    ) -> Result<PaginatedVideos, Error> {
        use crate::db_schema::videos::dsl::videos;
        use crate::db_schema::videos::*;

        let mut total_query = videos.into_boxed();
        if let Some(id_filter) = filter.id.clone() {
            total_query = total_query.filter(id.eq(id_filter));
        }

        let total = total_query
            .select(count(id))
            .first::<i64>(conn)
            .unwrap_or(0);

        let mut data_query = videos.into_boxed();
        if let Some(id_filter) = filter.id {
            data_query = data_query.filter(id.eq(id_filter));
        }

        let loaded_videos = data_query
            .offset(filter.offset)
            .limit(filter.first)
            .load::<Video>(conn)?;

        Ok(PaginatedVideos {
            first: filter.first,
            offset: filter.offset,
            total,
            results: loaded_videos,
        })
    }

    pub fn rate(&self, by_user: User, x: f64, conn: &mut PgConnection) -> Result<Video, Error> {
        use crate::db_schema::*;

        let new_rating = Rating {
            id: None,
            video_id: self.id.clone(),
            user_id: by_user.id,
            rating: x,
        };

        new_rating
            .clone()
            .insert_into(ratings::table)
            .on_conflict((video_id, user_id))
            .do_update()
            .set(ratings::rating.eq(new_rating.rating))
            .execute(conn)?;

        let average_rating = Rating::belonging_to(&self)
            .select(avg(ratings::rating))
            .first::<Option<f64>>(conn)
            .unwrap_or(None);

        let new_video = diesel::update(self)
            .set(videos::rating.eq(average_rating))
            .get_result::<Video>(conn)?;

        Ok(new_video)
    }
}

#[derive(InputObject)]
pub struct PaginatedVideosFilter {
    pub id: Option<String>,
    #[graphql(default = 25)]
    pub first: i64,
    #[graphql(default = 0)]
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PaginatedVideos {
    pub first: i64,
    pub offset: i64,
    pub total: i64,
    pub results: Vec<Video>,
}

#[derive(InputObject)]
pub struct AddVideoInput {
    pub id: String,
    pub platform: String,
}

#[derive(InputObject)]
pub struct RateVideoInput {
    pub id: String,
    pub rating: f64,
}
