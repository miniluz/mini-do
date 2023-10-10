use poem::web::Data;
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::task;

pub struct TaskApi;

#[derive(Debug, ApiResponse)]
pub enum GetResponse {
    /// Ok
    #[oai(status = 200)]
    Ok(Json<task::Model>),
    /// Task not found
    #[oai(status = 400)]
    TaskNotFound(PlainText<String>),
    /// Database error
    #[oai(status = 400)]
    DbErr,
}

#[OpenApi]
impl TaskApi {
    #[oai(path = "/", method = "get")]
    pub async fn get(&self, conn: Data<&DatabaseConnection>, id: Query<i32>) -> GetResponse {
        let id = id.0;
        let task_result = task::Entity::find_by_id(id).one(conn.0).await;

        match task_result {
            Ok(Some(task)) => GetResponse::Ok(Json(task)),
            Ok(None) => GetResponse::TaskNotFound(PlainText(format!("Task id {id} not found."))),
            Err(_) => GetResponse::DbErr,
        }
    }
}
