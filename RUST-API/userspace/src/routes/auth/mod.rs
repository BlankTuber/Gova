use rocket::Route;
pub mod login;
pub mod register;
pub mod logout;

#[options("/<_..>")]
fn all_options() {
    /* Empty */
}

pub fn routes() -> Vec<Route> {
    routes![register::register, login::login, logout::logout, all_options]
}