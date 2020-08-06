#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
mod utils;
use utils::response_data::ResponseForm;
use rocket_contrib::json::Json;



use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::{ Request, Data, Response};

#[get("/todo")]
fn todo() -> Json<ResponseForm> {
    // println!("{:?}", request.headers());
    let abc = ResponseForm::new();
    // println!("{:?}", a);
    Json(abc)
}

// 捕获404接口
#[catch(404)]
fn not_found(_req: &Request) -> Custom<Json<ResponseForm>> {
    let mut not_found = ResponseForm::new();
    not_found.url_not_found();
    // Json(not_found)
    Custom(Status::new(200, "reason"), Json(not_found))
}
// fn not_found(req: &Request) -> String {
//     format!("Sorry, '{}' is not a valid path.", req.uri())
// }

// 解决跨域等问题的
// use rocket::fairing::AdHoc;
use rocket::fairing::{Fairing, Info, Kind};
// use std::borrow::BorrowMut;
use rocket::http::Header;

struct MyFairing;
impl Fairing for MyFairing {
    // 添加的信息主要是拿来干嘛的
    fn info(&self) -> Info {
        Info {
            name: "My Custom Fairing",
            kind: Kind::Launch | Kind::Request | Kind::Response
        }
    }
    #[allow(unused_variables)]
    fn on_request(&self, request: &mut Request, data: &Data) {
        println!("aaaaaaaaaaaa");
        let origin = Header::new("Access-Control-Allow-Origin", "http://localhost:9527");
        let headers = Header::new("Access-Control-Allow-Headers", "GET, POST, OPTIONS, DELETE, PUT");
        let methods = Header::new("Access-Control-Allow-Methods", "*");
        request.add_header(origin);
        request.add_header(headers);
        request.add_header(methods);
        println!("{:?}",request.headers())
    }
    fn on_response(&self, _request: &Request, _response: &mut Response) {
        // println!("dadsadadadadadda");
        // println!("{:?}", request.headers());
        // let origin = Header::new("aaaa", "dsdadad");
        // response.set_header(origin);
    }
}
// 测试返回json数据
fn main() {
    rocket::ignite()
        .attach(MyFairing)
        .register(catchers![not_found])
        .mount("/", routes![todo])
        .launch();
}