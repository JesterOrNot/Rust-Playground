#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> std::io::Result<rocket::response::NamedFile> {
    rocket::response::NamedFile::open("www/index.html")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}