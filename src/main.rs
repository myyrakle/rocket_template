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

fn main() 
{
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}