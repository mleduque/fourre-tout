
use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDesc {

}

#[derive(Queryable)]
pub struct Model {
    pub id: i64,
    pub version: i64,
    pub name: String,
    pub old_version: Option<i64>,
    pub body: ModelDesc,
    pub published: DateTime<Utc>,
    pub first_published: DateTime<Utc>,
}