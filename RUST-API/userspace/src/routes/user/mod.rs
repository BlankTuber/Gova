use rocket::Route;
pub mod get_user;
pub mod update_email;
pub mod update_username;
pub mod update_profile;

#[options("/<_..>")]
fn all_options() {
    /* Empty */
}

pub fn routes() -> Vec<Route> {
    routes![
            get_user::get_user,
            update_email::update_email,
            update_username::update_username,
            update_profile::update_profile,
            all_options
        ]
}