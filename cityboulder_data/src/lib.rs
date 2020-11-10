#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel_migrations;

use self::schema::visitors::dsl::*;
use diesel::prelude::*;
use diesel_migrations::embed_migrations;

mod schema;
mod models;

embed_migrations!("migrations/");

pub use models::Visitors;
pub use models::VisitorLookup;

pub struct VisitorRepository {
    conn: diesel::SqliteConnection
}

impl VisitorRepository {
    pub fn new(path: &str) -> Self {
        VisitorRepository {
            conn: diesel::sqlite::SqliteConnection::establish(&path)
                .expect("Connection failed")
        }
        
    }
    pub fn run_migration(&self) {
        embedded_migrations::run(&self.conn)
            .expect("Schema migration failed");
    }

    pub fn save_visitor_lookup(&self, item: &VisitorLookup) {
        diesel::insert_into(self::schema::visitors::table)
            .values(item)
            .execute(&self.conn)
            .expect("Error saving new post");
    }
    pub fn load(&self) -> Vec<models::Visitors> {
        visitors//.filter(id.eq(1))
            //.limit(50)
            .load::<models::Visitors>(&self.conn)
            .expect("Error loading posts")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Local, DateTime};
    #[test]
    fn test() {
        let now =Local::now();
        let naive = chrono::NaiveDateTime::from_timestamp(3600, 0);
        let local = DateTime::<Local>::from_utc(naive, *now.offset());
        let utc = DateTime::<Utc>::from_utc(naive, Utc);
        dbg!(local, utc, local.naive_local(), utc.naive_local());
    }
}