#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_include_static_resources;

use std::collections::HashMap;
use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket_include_static_resources::StaticResponse;
use rocket::response::NamedFile;
use self::diesel::prelude::*;

pub mod db;
use db::models::Post;

#[derive(Serialize)]
struct TemplateContext {
    title: String,
    items: HashMap<&'static str, &'static str>
}

#[get("/favicon.ico")]
fn favicon() -> StaticResponse {
    static_response!("favicon")
}

#[get("/")]
fn index() -> Template {
    use db::schema::posts::dsl::*;
    let connection = db::establish_connection();
    let results = posts.limit(6)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    let mut v: Vec<HashMap<&'static str, String>> = Vec::new();

    for post in results {
        let mut m: HashMap<&'static str, String> = HashMap::new();
        m.insert("title", post.title);
        m.insert("body", post.body);
        m.insert("image", post.image);
        m.insert("url", post.url);
        m.insert("category", post.category);
        v.push(m);
    }
    
    let mut context: HashMap<&'static str, Vec<HashMap<&'static str, String>>> = HashMap::new();
    context.insert("items", v);

    Template::render("index", &context)
}

#[get("/projects/<name>")]
fn projects(name: String) -> Template {
    let mut page_values = HashMap::new();
    page_values.insert("Key", "Value");
    let context = TemplateContext { title: name, items: page_values };
    let template_dir = String::from("projects");

    let y = format!("{}/{}", template_dir, context.title);
    
    Template::render(y, &context)
}

#[get("/about")]
fn about() -> Template {
    let name = String::from("About");
    let context = TemplateContext { title: name, items: HashMap::new() };
    Template::render("about", &context)
}

#[get("/contact")]
fn contact() -> Template {
    let name = String::from("Contact");
    let context = TemplateContext { title: name, items: HashMap::new() };
    Template::render("contact", &context)
}

#[catch(400)]
fn bad_request(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/400", &map)
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

#[catch(403)]
fn forbidden(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/403", &map)
}

#[catch(500)]
fn server_error(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/500", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
	.attach(StaticResponse::fairing(|resources| {
		static_resources_initialize!(
			resources,
			"favicon", "favicon.ico"
		);
	}))
        .mount("/", routes![index, projects, about, contact])
	.mount("/", routes![favicon])
        .mount("/static", StaticFiles::from("static"))
        .register(catchers![not_found])
        .register(catchers![server_error])
}

fn main() {
    rocket().launch();
}
