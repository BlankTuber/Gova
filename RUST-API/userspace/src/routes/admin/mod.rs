use rocket::Route;
pub mod permissions;
pub mod roles;

#[options("/<_..>")]
fn all_options() {
    /* Empty */
}

pub fn routes() -> Vec<Route> {
    routes![
        roles::make_role,
        permissions::make_permission,
        all_options
    ]
}