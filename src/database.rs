use juniper::FieldResult;
use crate::schema::humans::NewHuman;
use crate::models::human::Human;

pub struct DatabasePool;

impl DatabasePool {
    pub fn get_connection(&self) -> FieldResult<DatabasePool> { Ok(DatabasePool) }
    pub fn find_human(&self, _id: &str) -> FieldResult<Human> { Err("")? }
    pub fn insert_human(&self, _human: &NewHuman) -> FieldResult<Human> { Err("")? }
}