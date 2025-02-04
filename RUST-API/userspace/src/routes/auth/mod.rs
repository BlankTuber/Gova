use rocket::Route;
pub mod login;
pub mod register;

#[options("/<_..>")]
fn all_options() {
    /* Empty */
}

pub fn routes() -> Vec<Route> {
    routes![register::register, login::login, all_options]
}