use super::schema::callbacks;
use chrono::prelude::*;

#[derive(Queryable, Serialize)]
pub struct Callback {
    id: i32,
    url: String,
    scheduled_date: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "callbacks"]
pub struct NewCallback<'a> {
    pub url: &'a str,
    pub scheduled_date: &'a DateTime<Utc>,
}
