#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template 
{
    let mut context = HashMap::new();
    context.insert("text", "Boom!!");
    Template::render("index", &context)
}

use rocket_contrib::serve::StaticFiles;

fn main() 
{
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
