use super::Bookmark;
use eza::fs::Dir;
use skim::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub fn open_selection_dialog(bookmarks: &Vec<Bookmark>) -> Result<String, String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .header(Some("Enter: select, Ctrl-x: delete, Ctrl-c: exit"))
        .bind(vec!["ctrl-x:accept"])
        .preview(Some("inline"))
        .layout("reverse")
        .build()
        .unwrap();

    //Create channel
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    // Send all bookmarks to skim through channel
    for item in bookmarks.to_vec() {
        tx.send(Arc::new(item)).unwrap();
    }

    let selected_bookmark = Skim::run_with(&options, Some(rx))
        .map(|out| match out.final_key {
            Key::Enter => out.selected_items,
            Key::Ctrl('x') => delete_selected_items(out.selected_items),
            _ => Vec::new(),
        })
        .unwrap_or_else(Vec::new);

    let result = selected_bookmark
        .get(0)
        .map(|selected_bookmark| selected_bookmark.output().into_owned());

    return Ok(result.unwrap_or_default());
}

fn delete_selected_items(items: Vec<Arc<dyn SkimItem>>) -> Vec<Arc<dyn SkimItem>> {
    for item in items.clone() {
        let bookmark = (*item)
            .as_any()
            .downcast_ref::<Bookmark>()
            .expect("Something wrong with downcast");
        super::remove_bookmark(&bookmark.id.unwrap());
    }
    items
}

// Preview

pub fn get_bookmark_preview(bookmark: &Bookmark) -> String {
    let type_icon = get_type_icon(&bookmark.path, true);
    let path_string = match &bookmark.path {
        Some(_) => format!(""),
        None => String::from(""),
    };

    let name_string = match &bookmark.name {
        Some(name) => format!("{} \n", name.to_string()),
        None => String::from(""),
    };

    let description_string = match &bookmark.description {
        Some(description) => format!("\n {}", description.to_string()),
        None => String::from("\n"),
    };
    let separator_string = String::from("\n-----------------------------------------");
    let preview_content = get_preview_content(bookmark);

    let result = format!(
        " {} {}{}{}{}{}",
        type_icon, name_string, description_string, path_string, separator_string, preview_content
    );
    result
}

fn get_preview_content(bookmark: &Bookmark) -> String {
    let path_string = match &bookmark.path {
        Some(path) => path.to_string(),
        None => String::from(""),
    };

    if path_string.is_empty() {
        return String::from("");
    }
    let path = Path::new(&path_string);

    let result = match path {
        path if path.is_file() => get_file_preview(&path_string),
        path if path.is_dir() => get_dir_preview(&path_string),
        _ => get_unknown_preview(&path_string),
    };
    result
}

fn get_file_preview(path_string: &String) -> String {
    let mut result = String::from("");
    let content = match std::fs::read_to_string(&path_string) {
        Ok(content) => content,
        Err(_) => String::from(""),
    };

    if !content.is_empty() {
        result = format!("\n{}", content); // add content
    }
    result
}

fn get_dir_preview(path_string: &String) -> String {
    // Create a Dir object using the eza library. For now it is used only to filter
    // out the dot files and the git ignored files

    let eza_dir = Dir::read_dir(PathBuf::from(path_string.clone()));
    if let Err(_) = eza_dir {
        return String::from("");
    }
    let eza_dir = eza_dir.unwrap();

    // Create a vector of files
    let files = eza_dir.files(eza::fs::DotFilter::JustFiles, None, true, true, true);

    // Add folder name to the result
    let directory_name = path_string.split("/").last().unwrap();
    let directory_icon = get_type_icon(&Option::from(path_string.to_string()), true);
    let mut result = format!("\n{} {}", directory_icon, directory_name);

    // Create a vector of filtered files
    let mut files_vec = Vec::new();
    for file in files {
        files_vec.push(file);
    }

    // Create compare functions for sorting
    let compare_names =
        |a: &eza::fs::File, b: &eza::fs::File| a.name.to_lowercase().cmp(&b.name.to_lowercase());
    let compare_types = |a: &eza::fs::File, b: &eza::fs::File| a.is_file().cmp(&b.is_file());
    files_vec.sort_by(compare_names);
    files_vec.sort_by(compare_types);

    for (i, file) in files_vec.iter().enumerate() {
        let type_icon = get_type_icon(&Option::from(file.path.to_str().unwrap().to_string()), true);
        let tree_char = if i == files_vec.len() - 1 {
            "└─"
        } else {
            "├─"
        };
        result = format!("{}\n{}{} {}", result, tree_char, type_icon, file.name);
    }

    result
}

fn get_unknown_preview(path_string: &String) -> String {
    format!("Could not preview: {}", path_string)
}

// Bookmark utils

pub fn get_bookmark_string(bookmark: &Bookmark) -> String {
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

pub fn get_bookmark_string_pretty(bookmark: &Bookmark) -> String {
    let type_icon = get_type_icon(&bookmark.path, true);
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

// UI utils

pub fn get_type_icon(path: &Option<String>, has_nerd_fonts: bool) -> String {
    let path = match &path {
        Some(path) => path.to_string(),
        None => String::from(""),
    };
    if path.is_empty() {
        match has_nerd_fonts {
            true => String::from("❔"),
            false => String::from("❔")
        }
    } else if std::path::Path::new(&path).is_file() {
        match has_nerd_fonts {
            true => String::from("📄"),
            false => String::from("📄")
            
        }
    } else if std::path::Path::new(&path).is_dir() {
        match has_nerd_fonts {
            true => String::from("📁"),
            false => String::from("📁")
        }
    } else {
        match has_nerd_fonts {
            true => String::from("❔"),
            false => String::from("❔")
        }
    }
}

