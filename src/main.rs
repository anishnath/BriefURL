use std::io::Write;
use BriefURL::establish_connection;
use BriefURL::insert_brief_url;
use BriefURL::schema::BriefUrl::dsl::BriefUrl;
use BriefURL::models::BriefUrlModel;
use diesel::prelude::*;
#[cfg(test)]
use diesel::debug_query;
fn main() {
    let connection = &mut establish_connection();
    let user_name = "Anish";
    let url = "https://8gwifi.org";
    let short_url = "tessbrown";
    let long_url = "tessbrown";

    if let Err(e) = insert_brief_url(connection, &user_name, &url, &short_url, &long_url) {
        writeln!(&mut std::io::stderr(), "Error: {}", e).unwrap();
        std::process::exit(1);
    }

    let _results = BriefUrl
        .filter(user_name.eq("Anish"))
        .limit(5)
        .load::<BriefUrlModel>(connection)
        .expect("Error loading posts");
}
