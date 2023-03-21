use diesel::prelude::*;

#[derive(Queryable, PartialEq, Debug)]
pub struct BriefUrlModel {
    pub id: i32,
    pub user_name: String,
    pub url: String,
    pub short_url: Option<String>,
    pub long_url: Option<String>,
    pub active: i32,
    pub clicks: i32,
    pub click_date: Option<String>,
    pub date_accessed: Option<String>,
}
