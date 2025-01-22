#[get("/user")]
pub fn get_user () -> &'static str {
    "Hello User!"
}