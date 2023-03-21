// @generated automatically by Diesel CLI.
use diesel::prelude::*;
diesel::table! {
    BriefUrl (id) {
        id -> Nullable<Integer>,
        user_name -> Text,
        url -> Text,
        short_url -> Nullable<Text>,
        long_url -> Nullable<Text>,
        clicks -> Integer,
        click_date -> Nullable<Timestamp>,
        active -> Integer,
        date_accessed -> Nullable<Timestamp>,
        date_created -> Timestamp,
    }
}
