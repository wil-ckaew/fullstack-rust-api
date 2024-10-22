use actix_web::{
    get, post, delete, patch, web::{Data, Json, scope, Query, Path, ServiceConfig}, HttpResponse, Responder
};
use serde_json::json;
use crate::{
    model::StudentModel,
    schema::{CreateStudentSchema, UpdateStudentSchema, FilterOptions},
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

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

#[patch("/students/{id}")]
async fn update_student_by_id(
    path: Path<Uuid>,
    body: Json<UpdateStudentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let student_id = path.into_inner();

    match sqlx::query_as!(
        StudentModel,
        "UPDATE students SET name = COALESCE($1, name), age = COALESCE($2, age) WHERE id = $3 RETURNING *",
        body.name.as_ref(),
        body.age.as_ref(),
        student_id
    )
    .fetch_one(&data.db)
    .await
    {
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

// Configuração das rotas para estudantes
pub fn config_students(conf: &mut ServiceConfig) {
    conf.service(create_student)
       .service(get_all_students)
       .service(get_student_by_id)
       .service(update_student_by_id)
       .service(delete_student_by_id);
}