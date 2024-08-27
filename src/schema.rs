use serde::{Deserialize, Serialize};
use uuid::Uuid; // Adicionado para o uso do tipo Uuid

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSchema {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateDocumentSchema {
    pub user_id: Uuid,
    pub doc_type: String,
    pub filename: String, // Adicionado para corresponder à criação de documentos
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

#[derive(Serialize, Deserialize, Debug)] // Adicionado Serialize para consistência
pub struct UpdateDocumentSchema {
    pub user_id: Option<Uuid>,
    pub doc_type: Option<String>,
    pub filename: Option<String>, // Adicionado para permitir atualização do filename
}
