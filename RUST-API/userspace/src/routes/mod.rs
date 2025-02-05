use rocket::Route;
pub mod auth;
pub mod admin;


pub fn auth_routes() -> Vec<Route> {
    auth::routes()
}

pub fn admin_routes() -> Vec<Route> {
    admin::routes()
}