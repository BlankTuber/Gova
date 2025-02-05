use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::{Json, json};
use rocket::serde::json::Value;

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Json<Value> {
    // Remove the 'auth_token' cookie
    cookies.remove_private(Cookie::from("auth_token"));
    Json(json!({ "message": "Logged out successfully" }))
}
