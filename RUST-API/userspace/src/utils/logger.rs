use anyhow::{Context, Result};
use chrono::Utc;
use rocket::serde::json::{json, Value};
use crate::models::log::{Log, CreateLog};
use uuid::Uuid;
use validator::Validate;

pub enum LogAction {
    Create,
    Update,
    Delete,
    Read,
    Custom(String),
}

impl ToString for LogAction {
    fn to_string(&self) -> String {
        match self {
            LogAction::Create => "create".to_string(),
            LogAction::Update => "update".to_string(),
            LogAction::Delete => "delete".to_string(),
            LogAction::Read => "read".to_string(),
            LogAction::Custom(s) => s.clone(),
        }
    }
}

pub struct LogBuilder {
    user_id: Option<Uuid>,
    action: LogAction,
    resource_type: String,
    resource_id: Option<String>,
    previous_state: Option<Value>,
    new_state: Option<Value>,
    additional_details: Option<Value>,
}

impl LogBuilder {
    pub fn new(action: LogAction, resource_type: &str) -> Self {
        LogBuilder {
            user_id: None,
            action,
            resource_type: resource_type.to_string(),
            resource_id: None,
            previous_state: None,
            new_state: None,
            additional_details: None,
        }
    }

    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_resource_id(mut self, resource_id: String) -> Self {
        self.resource_id = Some(resource_id);
        self
    }

    pub fn with_previous_state<T: serde::Serialize>(mut self, state: &T) -> Result<Self> {
        self.previous_state = Some(rocket::serde::json::to_value(state)?);
        Ok(self)
    }

    pub fn with_new_state<T: serde::Serialize>(mut self, state: &T) -> Result<Self> {
        self.new_state = Some(rocket::serde::json::to_value(state)?);
        Ok(self)
    }

    pub fn with_additional_details<T: serde::Serialize>(mut self, details: &T) -> Result<Self> {
        self.additional_details = Some(rocket::serde::json::to_value(details)?);
        Ok(self)
    }

    pub fn build(self) -> CreateLog {
        let mut details = json!({
            "resource_type": self.resource_type,
            "action_type": self.action.to_string(),
        });

        if let Some(resource_id) = self.resource_id {
            details.as_object_mut().unwrap().insert("resource_id".to_string(), json!(resource_id));
        }

        if let Some(previous_state) = self.previous_state {
            details.as_object_mut().unwrap().insert("previous_state".to_string(), previous_state);
        }

        if let Some(new_state) = self.new_state {
            details.as_object_mut().unwrap().insert("new_state".to_string(), new_state);
        }

        if let Some(additional_details) = self.additional_details {
            details.as_object_mut().unwrap().insert("additional_details".to_string(), additional_details);
        }

        CreateLog {
            user_id: self.user_id,
            action: format!("{}_{}", self.resource_type, self.action.to_string()),
            details,
        }
    }
}

pub async fn log_action(
    pool: &sqlx::PgPool,
    create_log: &CreateLog,
) -> Result<bool> {
    let log = Log {
        id: Uuid::new_v4(),
        user_id: create_log.user_id,
        action: create_log.action.clone(),
        details: create_log.details.clone(),
        created_at: Utc::now(),
    };

    create_log.validate()
        .context("Failed to validate log entry")?;

    let result = sqlx::query!(
        r#"
        INSERT INTO logs (id, user_id, action, details, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        log.id,
        log.user_id,
        log.action,
        log.details,
        log.created_at,
    )
    .execute(pool)
    .await
    .context("Failed to insert log entry into database")?;

    Ok(result.rows_affected() > 0)
}