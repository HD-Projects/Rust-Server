#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<post>")]
fn post_int(post: i16) -> String{
	format!("int")

}

#[get("/<post>", rank = 2)]
fn post_str(post: String) -> String{
	let header = "Header stuff";
	let author = "ad101-lab";
	let content = "Blog post content!";
	let footer = "Footer stuff";
	return format!("{} \nBlog post: \n author: {} \n Content: {} \n{} Name of page: {}", header, author, content, footer, name);
}

fn main() {
    rocket::ignite()
    	.mount("/", routes![index])
    	.mount("/blog", routes![post_int, post_str])
    	.launch();
}