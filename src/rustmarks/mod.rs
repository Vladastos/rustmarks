use rusqlite::Connection;
use std::path::PathBuf;
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

pub fn list_bookmarks() {
    let connection = Connection::open(DATABASE).unwrap();
    let bookmarks = get_bookmark_vec(&connection);
    for bookmark in bookmarks {
        let str = get_bookmark_string(&bookmark);
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
        get_bookmark_string_pretty(self).into()
    }
    fn output(&self) -> Cow<str> {
        self.path.as_ref().unwrap().into()
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(get_bookmark_preview(self).into())
    }
}

fn get_bookmark_string(bookmark: &Bookmark) -> String {
    let id = match bookmark.id {
        Some(id) => id.to_string(),
        None => String::from("None"),
    };

    let name = match &bookmark.name {
        Some(name) => name.to_string(),
        None => String::from("None"),
    };

    let path = match &bookmark.path {
        Some(path) => path.to_string(),
        None => String::from("None"),
    };

    let description = match &bookmark.description {
        Some(description) => description.to_string(),
        None => String::from("None"),
    };

    let str = format!(
        "id: {}, name: {}, path: {}, description: {}",
        id, name, path, description
    );
    str
}

fn get_bookmark_string_pretty(bookmark: &Bookmark) -> String {
    let type_icon = get_type_icon(&bookmark.path);
    let name = match &bookmark.name {
        Some(name) => name.to_string(),
        None => String::from(""),
    };

    let path = match &bookmark.path {
        Some(path) => {
            if name.is_empty() {
                path.to_string()
            } else {
                format!("")
            }
        }
        None => String::from(""),
    };
    let result = format!("{} {} {}", type_icon, name, path);
    result
}

fn get_type_icon(path: &Option<String>) -> String {
    let path = match &path {
        Some(path) => path.to_string(),
        None => String::from(""),
    };
    if path.is_empty() {
        String::from("❔")
    } else if std::path::Path::new(&path).is_file() {
        String::from("📄")
    } else if std::path::Path::new(&path).is_dir() {
        String::from("📁")
    } else {
        String::from("❔")
    }
}



fn get_bookmark_preview(bookmark: &Bookmark) -> String{
    let type_icon = get_type_icon(&bookmark.path);
    let path_string = match &bookmark.path {
        Some(path) => format!("\n Path: {}",path.to_string()),
        None => String::from(""),
    };

    let name_string = match &bookmark.name {
        Some(name) => format!("{} \n",name.to_string()),
        None => String::from(""),
    };

    let description_string = match &bookmark.description {
        Some(description) => format!("\n {} \n",description.to_string()),
        None => String::from(""),
    };
    let separator_string = String::from("\n-----------------------------------------");
    let preview_content = get_preview_content(bookmark);

    let result = format!(" {} {}{}{}{}{}",type_icon,name_string,description_string,path_string,separator_string,preview_content);
    result
}
fn get_preview_content(bookmark: &Bookmark) -> String {
    let path_string = match &bookmark.path {
        Some(path) => path.to_string(),
        None => String::from(""),
    };
    let mut result = String::from("");
    //check if path is a file
    if std::path::Path::new(&path_string).is_file() {
        let content = match std::fs::read_to_string(&path_string) {
            Ok(content) => content,
            Err(_) => String::from(""),
        };

        if !content.is_empty() {
            result = format!("\n{}", content); // add content
        }
    } else if std::path::Path::new(&path_string).is_dir() {
        let eza_dir =eza::fs::Dir::read_dir(PathBuf::from(path_string.clone()));
        if let Ok(eza_dir) = eza_dir {
            let files =eza_dir.files(eza::fs::DotFilter::JustFiles, None, true, true, false);

            // TODO: Sort directories first
            files.for_each(|file| {
                let type_icon = get_type_icon(&Option::from(file.path.to_str().unwrap().to_string()));
                let filename = file.name;
                result = format!("{}\n {} {}",result,type_icon, filename);
            });
        }
    }
    result
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
