use rocket::Route;
pub mod get_user;

#[options("/<_..>")]
fn all_options() {
    /* Empty */
}

pub fn routes() -> Vec<Route> {
    routes![get_user::get_user, all_options]
}