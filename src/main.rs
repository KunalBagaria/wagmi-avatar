#[macro_use] extern crate rocket;
use std::path::{Path};
use rocket::fs::{NamedFile, relative};

#[get("/<name>")]
pub async fn index(name: &str) -> Option<NamedFile> {
	let first_letter = String::from(name).to_lowercase().chars().nth(0).unwrap();
	if first_letter.is_alphabetic() {
		let path = Path::new(relative!("avatars")).join(format!("{}{}", first_letter, ".svg"));
		println!("{}", path.display());
		NamedFile::open(path).await.ok()
	} else {
		let path = Path::new(relative!("avatars")).join(format!("{}{}", "a", ".svg"));
		println!("{}", path.display());
		NamedFile::open(path).await.ok()
	}
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}