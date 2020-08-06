use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct ResponseForm {
    data: Option<Box<usize>>,
    err_code: i32,
    err_message: String
}

impl ResponseForm {
    pub fn new() -> ResponseForm {
        ResponseForm{
            data: Option::None,
            err_code: 200,
            err_message: "success".to_string()
        }
    }
    pub fn url_not_found(&mut self) {
        self.err_code = 404;
        self.err_message = "the route not found".to_string();
    }
}