pub mod auth;
// pub mod users;    // Future route groups

use rocket::Route;

/// Collects all routes in the application
pub fn routes() -> Vec<Route> {
    // Concatenate routes from all modules
    [
        auth::auth_routes(),
        // users::user_routes(),     // Future route groups
    ].concat()
}