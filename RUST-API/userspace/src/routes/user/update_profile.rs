use rocket::serde::json::{Json, json, Value};
use rocket::serde::json::serde_json::Map;
use rocket::State;
use rocket::http::Status;
use sqlx::PgPool;
use validator::Validate;
use sqlx::postgres::PgArguments;
use sqlx::Arguments;

use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::models::user_profile::UpdateUserProfile;
use crate::utils::logger::{LogBuilder, LogAction, log_action};

#[put("/profile", format = "json", data = "<profile_data>")]
pub async fn update_profile(
    user: AuthenticatedUser,
    pool: &State<PgPool>,
    profile_data: Json<UpdateUserProfile>
) -> Result<Json<Value>, Status> {
    let profile = profile_data.into_inner();

    // Validate the incoming profile data
    if profile.validate().is_err() {
        return Err(Status::BadRequest);
    }

    // Get current profile first
    let current_profile = sqlx::query!(
        r#"
        SELECT 
            display_name, bio, birth_date, 
            language, timezone, social_links
        FROM user_profiles 
        WHERE user_id = $1
        "#,
        user.user_id
    )
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?
    .ok_or(Status::NotFound)?;

    // Handle social links merging
    let merged_social_links = if let Some(ref new_links) = profile.social_links {
        let mut current_links = current_profile.social_links.clone()
            .unwrap_or_else(|| json!({}))
            .as_object()
            .cloned()
            .unwrap_or_else(Map::new);
        
        // Merge new links with current links
        if let Some(new_links_obj) = new_links.as_object() {
            for (key, value) in new_links_obj {
                if value.is_null() {
                    // Remove link if value is null
                    current_links.remove(key);
                } else {
                    // Update or add new link
                    current_links.insert(key.clone(), value.clone());
                }
            }
        }
        Some(Value::Object(current_links))
    } else {
        None
    };

    // Create state for logging
    let profile_for_log = UpdateUserProfile {
        display_name: profile.display_name.clone(),
        bio: profile.bio.clone(),
        birth_date: profile.birth_date,
        language: profile.language.clone(),
        timezone: profile.timezone.clone(),
        social_links: merged_social_links.clone(),
    };

    // Build dynamic update query
    let mut query_parts = Vec::new();
    let mut args = PgArguments::default();
    let mut param_index = 1;

    if profile.display_name.is_some() {
        query_parts.push(format!("display_name = ${}", param_index));
        args.add(profile.display_name);
        param_index += 1;
    }
    if profile.bio.is_some() {
        query_parts.push(format!("bio = ${}", param_index));
        args.add(profile.bio);
        param_index += 1;
    }
    if profile.birth_date.is_some() {
        query_parts.push(format!("birth_date = ${}", param_index));
        args.add(profile.birth_date);
        param_index += 1;
    }
    if profile.language.is_some() {
        query_parts.push(format!("language = ${}", param_index));
        args.add(profile.language);
        param_index += 1;
    }
    if profile.timezone.is_some() {
        query_parts.push(format!("timezone = ${}", param_index));
        args.add(profile.timezone);
        param_index += 1;
    }
    if merged_social_links.is_some() {
        query_parts.push(format!("social_links = ${}", param_index));
        args.add(merged_social_links);
        param_index += 1;
    }

    if query_parts.is_empty() {
        return Ok(Json(json!({
            "message": "No fields to update"
        })));
    }

    let query = format!(
        "UPDATE user_profiles SET {} WHERE user_id = ${}",
        query_parts.join(", "),
        param_index
    );

    args.add(user.user_id);

    // Perform the update
    let result = sqlx::query_with(&query, args)
        .execute(pool.inner())
        .await
        .map_err(|_| Status::InternalServerError)?;

    if result.rows_affected() == 0 {
        return Err(Status::NotFound);
    }

    // Create detailed log entry
    let log = LogBuilder::new(LogAction::Update, "user_profile")
        .with_user(user.user_id)
        .with_resource_id(user.user_id.to_string())
        .with_previous_state(&json!({
            "display_name": current_profile.display_name,
            "bio": current_profile.bio,
            "birth_date": current_profile.birth_date,
            "language": current_profile.language,
            "timezone": current_profile.timezone,
            "social_links": current_profile.social_links.clone()
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_new_state(&json!({
            "display_name": profile_for_log.display_name,
            "bio": profile_for_log.bio,
            "birth_date": profile_for_log.birth_date,
            "language": profile_for_log.language,
            "timezone": profile_for_log.timezone,
            "social_links": profile_for_log.social_links
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_additional_details(&json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "ip_address": "TODO: Add IP address capture"
        }))
        .map_err(|_| Status::InternalServerError)?
        .build();

    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "message": "Profile successfully updated!"
    })))
}