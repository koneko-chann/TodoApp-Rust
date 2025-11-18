use serde::{Deserialize, Serialize};


    #[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub task: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Status {
    PENDING,
    DONE,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]

pub struct TodoResponse {
    pub id: i64,
    pub task: String,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoType {
    pub task: Option<String>,
    pub status: Option<String>,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::PENDING => "PENDING",
            Status::DONE => "DONE",
        }
    }

    pub fn from_str(status: &str) -> Self {
        match status {
            "PENDING" => Status::PENDING,
            "DONE" => Status::DONE,
            _ => Status::PENDING,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ApiResponse {
    pub message: String,
    pub data: Vec<TodoResponse>,
}
