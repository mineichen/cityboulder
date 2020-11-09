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
            .limit(5)
            .load::<models::Visitors>(&self.conn)
            .expect("Error loading posts")
    }
}