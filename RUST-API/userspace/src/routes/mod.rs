use rocket::Route;
pub mod auth;
pub mod admin;
pub mod user;

pub fn auth_routes() -> Vec<Route> {
    auth::routes()
}

pub fn admin_routes() -> Vec<Route> {
    admin::routes()
}

pub fn user_routes() -> Vec<Route> {
    user::routes()
}