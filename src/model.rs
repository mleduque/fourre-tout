
use chrono::{DateTime, Utc};
use diesel::Queryable;
use serde::{Serialize, Deserialize};
use serde_json::{Value, from_value};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub r#type: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDesc {
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, Queryable, Serialize)]
pub struct Model {
    pub id: i64,
    pub version: i64,
    pub name: String,
    pub old_version: Option<i64>,
    pub body: Value,
    pub published: DateTime<Utc>,
    pub first_published: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParsedModel {
    pub id: i64,
    pub version: i64,
    pub name: String,
    pub old_version: Option<i64>,
    pub body: ModelDesc,
    pub published: DateTime<Utc>,
    pub first_published: DateTime<Utc>,
}

impl ParsedModel {
    pub fn parse(model: Model) -> Result<ParsedModel, serde_json::error::Error> {
        let model_desc: ModelDesc = from_value(model.body)?;
        Ok(ParsedModel {
            id: model.id,                       version: model.version,
            name: model.name,                   old_version: model.old_version,
            body: model_desc,                   published: model.published,
            first_published: model.first_published,
        })
    }
}