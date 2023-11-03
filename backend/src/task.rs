use chrono::{DateTime, Utc};
use poem::web::Data;
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait, IntoActiveValue,
};

use crate::entities::task;

pub struct TaskApi;

#[derive(Debug, Object)]
pub struct TaskBody {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub creation_time: DateTime<Utc>,
    pub due_time: DateTime<Utc>,
    pub done: bool,
}

impl From<TaskBody> for task::Model {
    fn from(value: TaskBody) -> Self {
        Self {
            id: value.id,
            title: value.title,
            text: value.text,
            creation_time: value.creation_time.naive_utc(),
            due_time: value.due_time.naive_utc(),
            done: value.done,
        }
    }
}

#[derive(Debug, ApiResponse)]
pub enum GetTaskResponse {
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

#[derive(Debug, ApiResponse)]
pub enum GetTasksResponse {
    /// Ok
    #[oai(status = 200)]
    Ok(Json<Vec<task::Model>>),
    /// Database error
    #[oai(status = 400)]
    DbErr,
}

#[derive(Debug, ApiResponse)]
pub enum PatchTaskResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    TaskNotFound,
    #[oai(status = 400)]
    DbErr,
}

#[derive(Debug, ApiResponse)]
pub enum PostTaskResponse {
    #[oai(status = 200)]
    Ok(Json<i32>),
    #[oai(status = 400)]
    DbErr,
}

#[derive(Debug, ApiResponse)]
pub enum DeleteTaskResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    DbErr,
}

#[OpenApi]
impl TaskApi {
    #[oai(path = "/task", method = "get")]
    pub async fn get_task(
        &self,
        conn: Data<&DatabaseConnection>,
        id: Query<i32>,
    ) -> GetTaskResponse {
        let id = id.0;
        let task_result = task::Entity::find_by_id(id).one(conn.0).await;

        match task_result {
            Ok(Some(task)) => GetTaskResponse::Ok(Json(task)),
            Ok(None) => {
                GetTaskResponse::TaskNotFound(PlainText(format!("Task id {id} not found.")))
            }
            Err(_) => GetTaskResponse::DbErr,
        }
    }

    #[oai(path = "/tasks", method = "get")]
    pub async fn get_tasks(&self, conn: Data<&DatabaseConnection>) -> GetTasksResponse {
        let tasks_result = task::Entity::find().all(conn.0).await;

        match tasks_result {
            Ok(tasks) => GetTasksResponse::Ok(Json(tasks)),
            Err(_) => GetTasksResponse::DbErr,
        }
    }

    #[oai(path = "/task", method = "patch")]
    pub async fn update_task(
        &self,
        conn: Data<&DatabaseConnection>,
        task: Json<TaskBody>,
    ) -> PatchTaskResponse {
        let task: task::Model = task.0.into();
        let user_task = task::ActiveModel {
            id: task.id.into_active_value(),
            title: task.title.into_active_value(),
            text: task.text.into_active_value(),
            creation_time: task.creation_time.into_active_value(),
            due_time: task.due_time.into_active_value(),
            done: task.done.into_active_value(),
        };
        let update_result = user_task.update(conn.0).await;
        match update_result {
            Ok(_) => PatchTaskResponse::Ok,
            Err(dberr) => match dberr {
                DbErr::RecordNotFound(_) => PatchTaskResponse::TaskNotFound,
                _ => PatchTaskResponse::DbErr,
            },
        }
    }

    #[oai(path = "/task", method = "post")]
    pub async fn create_task(
        &self,
        conn: Data<&DatabaseConnection>,
        task: Json<TaskBody>,
    ) -> PostTaskResponse {
        let task: task::Model = task.0.into();
        let user_task = task::ActiveModel {
            id: ActiveValue::NotSet,
            title: task.title.into_active_value(),
            text: task.text.into_active_value(),
            creation_time: task.creation_time.into_active_value(),
            due_time: task.due_time.into_active_value(),
            done: task.done.into_active_value(),
        };
        let db_result = task::Entity::insert(user_task).exec(conn.0).await;
        match db_result {
            Ok(insert_result) => PostTaskResponse::Ok(Json(insert_result.last_insert_id)),
            Err(_) => PostTaskResponse::DbErr,
        }
    }

    #[oai(path = "/task", method = "delete")]
    pub async fn delete_task(
        &self,
        conn: Data<&DatabaseConnection>,
        id: Query<i32>,
    ) -> DeleteTaskResponse {
        let user_task = task::ActiveModel {
            id: id.0.into_active_value(),
            ..Default::default()
        };

        let delete_result = user_task.delete(conn.0).await;
        match delete_result {
            Ok(_) => DeleteTaskResponse::Ok,
            Err(_) => DeleteTaskResponse::DbErr,
        }
    }
}
