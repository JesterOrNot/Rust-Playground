#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use std::io::Result;
use rocket::response::NamedFile;

#[get("/")]
fn index() -> Result<NamedFile> {
    NamedFile::open("www/index.html")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
