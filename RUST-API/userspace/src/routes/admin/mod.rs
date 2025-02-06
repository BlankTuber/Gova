use rocket::Route;
pub mod permissions;
pub mod roles;
pub mod users;

#[options("/<_..>")]
fn all_options() {
    /* Empty */
}

pub fn routes() -> Vec<Route> {
    routes![
        roles::make_role,
        roles::delete_role,
        roles::get_all_roles,
        roles::assign_permission_to_role,
        permissions::make_permission,
        permissions::delete_permission,
        permissions::get_all_permissions,
        users::assign_role_to_user,
        users::get_all_users,
        users::delete_user,
        users::remove_user_from_role,
        all_options
    ]
}