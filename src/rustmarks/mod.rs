use std::{path::PathBuf, process::exit};

use rusqlite::Connection;
use skim::prelude::*;
mod sqlite_repository;
mod ui;
const DATABASE: &str = "/home/vlad/.config/rustmarks/rustmarks.db";

pub fn main() {
    let connection = Connection::open(DATABASE).unwrap();
    let bookmarks = get_bookmark_vec(&connection);
    let result = ui::open_selection_dialog(&bookmarks);
    if let Err(e) = result {
        println!("Error: {}", e);
        return;
    }
    println!("{}", result.unwrap());
}

pub fn print_command() {
    let connection = Connection::open(DATABASE).unwrap();
    let bookmarks = get_bookmark_vec(&connection);
    let result = ui::open_selection_dialog(&bookmarks);
    if let Err(e) = result {
        println!("Error: {}", e);
        return;
    }
    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    let mut command = String::from("");
    let path = PathBuf::from(result.unwrap());
    match path {
        p if p.is_file() => {
            command = format!("{} {}", editor, p.display());
        }
        p if p.is_dir() => {
            command = format!("cd {}", p.display());
        }
        _ => {}
    }

    println!("{}", command);
}

pub fn add_bookmark(name: &Option<String>, path: &String, description: &Option<String>) {
    let bookmark = parse_bookmark(&None, name, path, description);
    let connection = Connection::open(DATABASE).unwrap();
    let result = sqlite_repository::add_bookmark(&bookmark, &connection);
    if let Err(_e) = result {
        return;
    }
    if result.is_ok() {
        println!("Bookmark added");
    }
}

pub fn list_bookmarks(paths:bool) {
    let connection = Connection::open(DATABASE).unwrap();
    let bookmarks = get_bookmark_vec(&connection);
    for bookmark in bookmarks {
        let str: String;
        if !paths {
            str = ui::get_bookmark_string(&bookmark);
        } else {
            str = bookmark.path.unwrap_or("".to_string());   
        }
        println!("{}", str);
    }
}

pub fn remove_bookmark(id: &i32) {
    let connection = Connection::open(DATABASE).unwrap();
    // Check if bookmark exists
    let result = sqlite_repository::get_bookmark(id, &connection);
    if let Err(_) = result {
        println!("error removing bookmark: Bookmark with id {} not found", id);
        return;
    }
    let result = sqlite_repository::remove_bookmark(&id, &connection);
    if let Err(e) = result {
        println!("error removing bookmark: {}", e);
        return;
    }
    println!("Bookmark removed");
}

pub fn update_bookmark(
    id: &i32,
    name: &Option<String>,
    path: &Option<String>,
    description: &Option<String>,
) {
    let connection: Connection = Connection::open(DATABASE).unwrap();
    let old_bookmark_res = sqlite_repository::get_bookmark(id, &connection);
    if let Err(e) = old_bookmark_res {
        println!("Error: {}", e);
        return;
    }
    let old_bookmark = old_bookmark_res.unwrap();

    let new_bookmark_path = match path {
        Some(path) => Some(path.to_string()),
        None => old_bookmark.path,
    };

    // Assemble new bookmark
    let new_name = match name {
        Some(name) => Some(name.to_string()),
        None => old_bookmark.name,
    };

    let new_description = match description {
        Some(description) => Some(description.to_string()),
        None => old_bookmark.description,
    };

    let new_bookmark = parse_bookmark(
        &Option::from(id.to_owned()),
        &new_name,
        &new_bookmark_path.unwrap(),
        &new_description,
    );
    let result = sqlite_repository::update_bookmark(&id, &new_bookmark, &connection);

    if let Err(e) = result {
        println!("Error: {}", e);
        return;
    }
    println!("Bookmark updated");
}

pub fn check_bookmark(path: &Option<String>) {
    let connection = Connection::open(DATABASE).unwrap();
    let path_string = path.clone().unwrap_or(std::env::current_dir().unwrap().display().to_string());
    let result = sqlite_repository::check_bookmark(&path_string, &connection);
    if let Err(e) = result {
        println!("Error: {}", e);
        return;
    }
    match result.unwrap() {
        true => exit(0),
        false => exit(1),
    }
}

fn get_bookmark_vec(connection: &Connection) -> Vec<Bookmark> {
    let bookmarks_res = sqlite_repository::list_bookmarks(&connection);
    let bookmarks = match bookmarks_res {
        Ok(bookmarks) => bookmarks,
        Err(e) => {
            println!("Error: {}", e);
            return Vec::new();
        }
    };

    bookmarks
}

#[derive(Debug, Clone)]
struct Bookmark {
    id: Option<i32>,
    name: Option<String>,
    path: Option<String>,
    description: Option<String>,
}
impl SkimItem for Bookmark {
    fn text(&self) -> Cow<str> {
        ui::get_bookmark_string_pretty(self).into()
    }
    fn output(&self) -> Cow<str> {
        self.path.as_ref().unwrap().into()
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(ui::get_bookmark_preview(self).into())
    }
}

fn parse_bookmark(
    id: &Option<i32>,
    name: &Option<String>,
    path: &String,
    description: &Option<String>,
) -> Bookmark {
    let absolute_path = std::fs::canonicalize(path);

    if let Err(e) = absolute_path {
        println!("Error: {}", e);
        return Bookmark {
            id: None,
            name: None,
            path: None,
            description: None,
        };
    }

    let path_opt = absolute_path.unwrap();

    let path = match path_opt.to_str() {
        Some(p) => Some(p.to_string()),
        None => None,
    };

    Bookmark {
        id: id.clone(),
        name: name.clone(),
        path: Some(path.unwrap().to_string()),
        description: description.clone(),
    }
}
