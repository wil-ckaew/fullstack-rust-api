use serde::{Deserialize, Serialize};
use uuid::Uuid; // Adicionado para o uso do tipo Uuid

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSchema {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateStudentSchema {
    pub name: String,
    pub age: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDocumentSchema {
    pub student_id: Uuid, // Altere aqui de user_id para student_id
    pub doc_type: String,
    pub filename: String, // Adicionado para corresponder à criação de documentos
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub username: String,
    pub hashed_password: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct CreateParentSchema {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Deserialize)]
pub struct CreatePhotoSchema {
    pub student_id: Uuid,
    pub filename: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateVideoSchema {
    pub student_id: Uuid,
    pub filename: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFileMetadataSchema {
    pub user_id: Uuid, // Pode ser opcional se o arquivo não estiver associado a um usuário
    pub file_type: String,     // Deve ser 'video' ou 'photo'
    pub filename: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLogSchema {
    pub user_id: Uuid, // Pode ser opcional se o arquivo não estiver associado a um usuário
    pub action: String,     // Deve ser 'video' ou 'photo'
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSchema {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateStudentSchema {
    pub name: Option<String>,
    pub age: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)] // Adicionado Serialize para consistência
pub struct UpdateDocumentSchema {
    pub student_id: Option<Uuid>, // Altere aqui de user_id para student_id
    pub doc_type: Option<String>,
    pub filename: Option<String>, // Adicionado para permitir atualização do filename
}

#[derive(Serialize, Deserialize, Debug)] // Adicionado Serialize para consistência
pub struct UpdateUserSchema {
    pub username: Option<String>,
    pub hashed_password: Option<String>,
    pub role: Option<String>, // Adicionado para permitir atualização do filename
}

#[derive(Serialize, Deserialize, Debug)] // Adicionado Serialize para consistência
pub struct UpdateParentSchema {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>, // Adicionado para permitir atualização do filename
}

#[derive(Deserialize)]
pub struct UpdatePhotoSchema {
    pub student_id: Option<Uuid>,
    pub filename: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateVideoSchema {
    pub student_id: Option<Uuid>,
    pub filename: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFileMetadataSchema {
    pub user_id: Option<Uuid>, // Pode ser opcional se o arquivo não estiver associado a um usuário
    pub file_type: Option<String>,     // Deve ser 'video' ou 'photo'
    pub filename: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateLogSchema {
    pub user_id: Option<Uuid>, // Pode ser opcional se o arquivo não estiver associado a um usuário
    pub action: Option<String>,     // Deve ser 'video' ou 'photo'
    pub description: Option<String>,
}