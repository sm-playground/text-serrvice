use crate::api_error::ApiError;
use crate::db_connection;
use crate::schema::textinfo;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// This represents a TextInfo object pulled from the database
#[derive(Serialize, Deserialize, Queryable)]
pub struct TextInfo {
    pub id: i64,
    pub token: String,
    pub text: String,
}

/// This represents a TextInfo object inserted into the database
#[derive(Deserialize, Insertable)]
#[table_name = "textinfo"]
pub struct InsertableTextInfo {
    pub token: String,
    pub text: String,
}

impl TextInfo {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db_connection::connection()?;

        let all_text_info = textinfo::table
            .load::<TextInfo>(&conn)?;

        Ok(all_text_info)
    }

    pub fn find(id: i64) -> Result<Self, ApiError> {
        let conn = db_connection::connection()?;

        let text_info = textinfo::table
            .filter(textinfo::id.eq(id))
            .first(&conn)?;

        Ok(text_info)
    }

/*
    pub fn create(text_info: InsertableTextInfo) -> Result<Self, ApiError> {
        let conn = db_connection::connection()?;

        let text_info = TextInfo::from(text_info);
        let text_info = diesel::insert_into(textinfo::table)
            .values(text_info)
            .get_result(&conn)?;

        Ok(text_info)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::update(user::table)
            .filter(user::id.eq(id))
            .set(user)
            .get_result(&conn)?;

        Ok(user)
    }
*/

    pub fn delete(id: i64) -> Result<usize, ApiError> {
        let conn = db_connection::connection()?;

        let res = diesel::delete(
                textinfo::table
                    .filter(textinfo::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}

impl From<InsertableTextInfo> for TextInfo {
    fn from(text_info: InsertableTextInfo) -> Self {
        TextInfo {
            id: 1,
            token: text_info.token,
            text: text_info.text,
        }
    }
}