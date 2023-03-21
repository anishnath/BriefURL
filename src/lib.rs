pub mod models;
pub mod schema;

#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;
use diesel::prelude::*;
#[cfg(test)]
use diesel::sqlite::Sqlite;
use dotenvy::dotenv;
use std::env;
use models::BriefUrlModel;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn search_brief_url(
//     conn: &mut &SqliteConnection,
//     search_term: &str,
// ) -> Result<Vec<BriefUrlModel>, diesel::result::Error> {
//
//     use schema::BriefUrl::dsl::*;
//     //use diesel::RunQueryDsl::load;
//
//     BriefUrl
//         .filter(
//             user_name
//                 .like(format!("%{}%", search_term))
//                 .or(long_url.like(format!("%{}%", search_term)))
//                 .or(short_url.like(format!("%{}%", search_term))),
//         )
//         .load::<BriefUrlModel>(conn)
// }

pub fn insert_brief_url(
    conn: &mut SqliteConnection,
    p_user_name: &str,
    p_url: &str,
    p_short_url: &str,
    p_long_url: &str,
) -> Result<usize, diesel::result::Error> {
    use schema::BriefUrl::dsl::*;

    let new_row = (
        user_name.eq(p_user_name),
        url.eq(p_url),
        short_url.eq(p_short_url),
        long_url.eq(p_long_url),
        clicks.eq(0),
    );

    let num_rows_inserted = insert_into(BriefUrl).values(new_row).execute(conn);

    match num_rows_inserted {
        Ok(num_rows) => {
            println!("Inserted {} rows", num_rows);
            Ok(num_rows)
        }
        Err(e) => {
            eprintln!("Error inserting row: {:?}", e);
            Err(e)
        }
    }
}

#[test]
fn examine_sql_insert_brief_url() {
    use schema::BriefUrl::dsl::*;

    let query = insert_into(BriefUrl).values((
        user_name.eq("Anish"),
        url.eq("https://8gwifi.org"),
        short_url.eq("tessbrown"),
        long_url.eq("tessbrown"),
        clicks.eq(0),
    ));
    let sql = "INSERT INTO `BriefUrl` (`user_name`, `url`, `short_url`, `long_url`, `clicks`) VALUES (?, ?, ?, ?, ?) \
               -- binds: [\"Anish\", \"https://8gwifi.org\", \"tessbrown\", \"tessbrown\", 0]";
    assert_eq!(sql, debug_query::<Sqlite, _>(&query).to_string());
}
