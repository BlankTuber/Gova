pub mod register;

pub fn auth_routes() -> Vec<rocket::Route> {
    routes![
        register::register,
        // login::login,
        // logout::logout,
        // etc.
    ]
}