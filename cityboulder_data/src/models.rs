use super::schema::visitors;

#[derive(Queryable, PartialEq, Debug, Serialize)]
pub struct Visitors {
    pub id: i32,
    pub guest_count: i32,
    pub recorded_at: chrono::NaiveDateTime
}


#[derive(Insertable)]
#[table_name="visitors"]
pub struct VisitorLookup {
    pub guest_count: i32,
    pub recorded_at: chrono::NaiveDateTime,
}

