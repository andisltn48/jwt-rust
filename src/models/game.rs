

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub field_name: String,
    pub address: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CreateFieldSchema {
    pub field_name: String,
    pub address: String,
    pub date: chrono::DateTime<chrono::Utc>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UpdateFieldSchema {
    pub field_name: String,
    pub address: String,
    pub date: chrono::DateTime<chrono::Utc>
}