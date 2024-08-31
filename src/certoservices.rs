use actix_web::{
    get, post, delete, patch, web::{Data, Json, scope, Query, Path, ServiceConfig}, HttpResponse, Responder
};
use serde_json::json;
use crate::{
    model::{TaskModel, DocumentModel, UserModel, StudentModel},
    schema::{
                CreateTaskSchema, 
                CreateDocumentSchema, 
                UpdateTaskSchema, 
                UpdateDocumentSchema,
                CreateUserSchema, 
                UpdateUserSchema, 
                CreateStudentSchema, 
                UpdateStudentSchema,
                FilterOptions
            },
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

// Endpoint de verificação de saúde
#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health check: API is up and running smoothly.";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

// Endpoint para criar uma tarefa
#[post("/tasks")]
async fn create_task(
    body: Json<CreateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO tasks (title, content)
        VALUES ($1, $2)
        RETURNING id, title, content, created_at
    "#;

    match sqlx::query_as::<_, TaskModel>(query)
        .bind(&body.title)
        .bind(&body.content)
        .fetch_one(&data.db)
        .await
    {
        Ok(task) => {
            let response = json!({
                "status": "success",
                "task": {
                    "id": task.id,
                    "title": task.title,
                    "content": task.content,
                    "created_at": task.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create task: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para criar um documento
#[post("/documents")]
async fn create_document(
    body: Json<CreateDocumentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO documents (user_id, doc_type, filename)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, doc_type, filename, created_at
    "#;

    // Gera um nome de arquivo com base no UUID
    let filename = format!("document_{}.jpg", Uuid::new_v4());

    match sqlx::query_as::<_, DocumentModel>(query)
        .bind(&body.user_id)
        .bind(&body.doc_type)
        .bind(&filename)
        .fetch_one(&data.db)
        .await
    {
        Ok(document) => {
            let response = json!({
                "status": "success",
                "document": {
                    "id": document.id,
                    "user_id": document.user_id,
                    "doc_type": document.doc_type,
                    "filename": document.filename,
                    "created_at": document.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create document: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint de criação de tarefa
#[post("/users")]
async fn create_user(
    body: Json<CreateUserSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO users (username, hashed_password, role)
        VALUES ($1, $2, $3)
        RETURNING id, username, hashed_password, role, created_at
    "#;

    match sqlx::query_as::<_, UserModel>(query)
        .bind(&body.username)
        .bind(&body.hashed_password)
        .bind(&body.role)
        .fetch_one(&data.db)
        .await
    {
        Ok(user) => {
            let response = json!({
                "status": "success",
                "user": {
                    "id": user.id,
                    "username": user.username,
                    "hashed_password": user.hashed_password,
                    "role": user.role,
                    "created_at": user.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create user: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}


// Endpoint de criação de tarefa
#[post("/students")]
async fn create_student(
    body: Json<CreateStudentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO students (name, age)
        VALUES ($1, $2)
        RETURNING id, name, age, created_at
    "#;

    match sqlx::query_as::<_, StudentModel>(query)
        .bind(&body.name)
        .bind(&body.age)
        .fetch_one(&data.db)
        .await
    {
        Ok(student) => {
            let response = json!({
                "status": "success",
                "student": {
                    "id": student.id,
                    "name": student.name,
                    "age": student.age,
                    "created_at": student.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create student: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}


// Endpoint para listar todas as tarefas
#[get("/tasks")]
pub async fn get_all_tasks(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(tasks) => {
            let response = json!({
                "status": "success",
                "tasks": tasks
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get tasks: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para obter uma tarefa por ID
#[get("/tasks/{id}")]
async fn get_task_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks WHERE id = $1",
        task_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(task) => {
            let response = json!({
                "status": "success",
                "task": task
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get task: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para listar todos os documentos
#[get("/documents")]
pub async fn get_all_documents(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        DocumentModel,
        "SELECT * FROM documents ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(documents) => {
            let response = json!({
                "status": "success",
                "documents": documents
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get documents: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para obter um documento por ID
#[get("/documents/{id}")]
pub async fn get_document_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let document_id = path.into_inner();

    match sqlx::query_as!(
        DocumentModel,
        "SELECT * FROM documents WHERE id = $1",
        document_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(document) => {
            let response = json!({
                "status": "success",
                "document": document
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get document: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para listar todas as tarefas
#[get("/users")]
pub async fn get_all_users(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        UserModel,
        "SELECT * FROM users ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(users) => {
            let response = json!({
                "status": "success",
                "users": users
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get users: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para obter uma tarefa por ID
#[get("/users/{id}")]
async fn get_user_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let user_id = path.into_inner();

    match sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => {
            let response = json!({
                "status": "success",
                "user": user
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get user: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para listar todas as tarefas
#[get("/students")]
pub async fn get_all_students(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        StudentModel,
        "SELECT * FROM students ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(students) => {
            let response = json!({
                "status": "success",
                "students": students
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get students: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para obter uma tarefa por ID
#[get("/students/{id}")]
async fn get_student_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let student_id = path.into_inner();

    match sqlx::query_as!(
        StudentModel,
        "SELECT * FROM students WHERE id = $1",
        student_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(student) => {
            let response = json!({
                "status": "success",
                "student": student
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get student: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para excluir uma tarefa por ID
#[delete("/users/{id}")]
async fn delete_user_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let user_id = path.into_inner();

    match sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete user: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para excluir uma tarefa por ID
#[delete("/students/{id}")]
async fn delete_student_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let student_id = path.into_inner();

    match sqlx::query!("DELETE FROM students WHERE id = $1", student_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete student: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}


// Endpoint para excluir uma tarefa por ID
#[delete("/tasks/{id}")]
async fn delete_task_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete task: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para excluir um documento por ID
#[delete("/documents/{id}")]
async fn delete_document_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let document_id = path.into_inner();

    match sqlx::query!("DELETE FROM documents WHERE id = $1", document_id)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete document: {:?}", err)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para atualizar uma tarefa por ID
#[patch("/tasks/{id}")]
async fn update_task_by_id(
    path: Path<Uuid>,
    body: Json<UpdateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {
    let task_id = path.into_inner();

    // Recuperar a tarefa existente
    match sqlx::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(task) => {
            // Atualizar a tarefa
            let update_result = sqlx::query_as!(
                TaskModel,
                "UPDATE tasks SET title = COALESCE($1, title), content = COALESCE($2, content) WHERE id = $3 RETURNING *",
                body.title.as_ref(),
                body.content.as_ref(),
                task_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_task) => {
                    let response = json!({
                        "status": "success",
                        "task": updated_task
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update task: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("Task not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

// Endpoint para atualizar um documento por ID
#[patch("/documents/{id}")]
async fn update_document_by_id(
    path: Path<Uuid>,
    body: Json<UpdateDocumentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let document_id = path.into_inner();

    // Recuperar o documento existente
    match sqlx::query_as!(
        DocumentModel,
        "SELECT * FROM documents WHERE id = $1",
        document_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(_document) => {
            // Atualizar o documento
            let update_result = sqlx::query_as!(
                DocumentModel,
                "UPDATE documents SET user_id = COALESCE($1, user_id), doc_type = COALESCE($2, doc_type) WHERE id = $3 RETURNING *",
                body.user_id.as_ref(),
                body.doc_type.as_ref(),
                document_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_document) => {
                    let response = json!({
                        "status": "success",
                        "document": updated_document
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update document: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("Document not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

// Endpoint para atualizar uma tarefa por ID
#[patch("/users/{id}")]
async fn update_user_by_id(
    path: Path<Uuid>,
    body: Json<UpdateUserSchema>,
    data: Data<AppState>
) -> impl Responder {
    let user_id = path.into_inner();

    // Recuperar a tarefa existente
    match sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(user) => {
            // Atualizar a tarefa
            let update_result = sqlx::query_as!(
                UserModel,
                "UPDATE users SET username = COALESCE($1, username), hashed_password = COALESCE($2, hashed_password), role = COALESCE($3, role) WHERE id = $4 RETURNING *",
                body.username.as_ref(),
                body.hashed_password.as_ref(),
                body.role.as_ref(),
                user_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_user) => {
                    let response = json!({
                        "status": "success",
                        "user": updated_user
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update task: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("User not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

// Endpoint para atualizar uma tarefa por ID
#[patch("/students/{id}")]
async fn update_student_by_id(
    path: Path<Uuid>,
    body: Json<UpdateStudentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let student_id = path.into_inner();

    // Recuperar a tarefa existente
    match sqlx::query_as!(StudentModel, "SELECT * FROM students WHERE id = $1", student_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(student) => {
            // Atualizar a tarefa
            let update_result = sqlx::query_as!(
                StudentModel,
                "UPDATE students SET name = COALESCE($1, name), age = COALESCE($2, age) WHERE id = $3 RETURNING *",
                body.name.as_ref(),
                body.age.as_ref(),
                student_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_student) => {
                    let response = json!({
                        "status": "success",
                        "student": updated_student
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let response = json!({
                        "status": "error",
                        "message": format!("Failed to update student: {:?}", update_error)
                    });
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        Err(fetch_error) => {
            let response = json!({
                "status": "error",
                "message": format!("Student not found: {:?}", fetch_error)
            });
            HttpResponse::NotFound().json(response)
        }
    }
}

// Configuração das rotas
pub fn config(conf: &mut ServiceConfig) {
    conf.service(
        scope("/api")
            .service(health_checker)
            .service(create_task)
            .service(create_document)
            .service(create_user)
            .service(create_student)            
            .service(get_all_tasks)
            .service(get_task_by_id)
            .service(get_all_documents)
            .service(get_document_by_id)
            .service(get_all_users)
            .service(get_user_by_id)   
            .service(get_all_students)
            .service(get_student_by_id)                    
            .service(delete_task_by_id)
            .service(delete_document_by_id)
            .service(update_task_by_id)
            .service(update_document_by_id)
            .service(delete_user_by_id)
            .service(update_user_by_id)
            .service(delete_student_by_id)
            .service(update_student_by_id)
    );
}
