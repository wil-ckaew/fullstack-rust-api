use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TaskModel {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DocumentModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub doc_type: String,
    pub filename: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct StudentModel {
    pub id: Uuid,
    pub name: String,
    pub age: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ParentModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,  // Ajustado para Option<DateTime<Utc>> para lidar com valores nulos
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct StudentParentModel {
    pub student_id: Uuid,
    pub parent_id: Uuid,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct VideoModel {
    pub id: Uuid,
    pub student_id: Uuid,
    pub filename: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct PhotoModel {
    pub id: Uuid,
    pub student_id: Uuid,
    pub filename: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub role: String, // Consider using an enum for role if it's limited to specific values
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct FileMetadataModel {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub file_type: String,
    pub filename: String,
    pub description: Option<String>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct LogModel {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub description: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
}
