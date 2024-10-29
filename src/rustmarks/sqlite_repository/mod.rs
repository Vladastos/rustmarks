use super::Bookmark;

pub fn add_bookmark(
    bookmark: &Bookmark,
    connection: &rusqlite::Connection,
) -> Result<(), rusqlite::Error> {
    create_table(connection)?;

    // Check if bookmark with given path exists
    let mut stmt = connection.prepare("SELECT * FROM bookmarks WHERE path = ?")?;
    let bookmark_check = stmt.query_row(&[&bookmark.path], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            description: row.get(3)?,
        })
    });
    if bookmark_check.is_ok() {
        println!(
            "Bookmark with path {} already exists",
            bookmark_check.unwrap().path.to_owned().unwrap()
        );
        return Err(rusqlite::Error::InvalidQuery);
    }

    let mut stmt =
        connection.prepare("INSERT INTO bookmarks (name, path, description) VALUES (?, ?, ?)")?;
    stmt.execute(&[&bookmark.name, &bookmark.path, &bookmark.description])?;
    Ok(())
}

pub fn list_bookmarks(connection: &rusqlite::Connection) -> Result<Vec<Bookmark>, rusqlite::Error> {
    create_table(connection)?;
    let mut bookmarks_vec: Vec<Bookmark> = Vec::new();
    let mut stmt = connection.prepare("SELECT * FROM bookmarks")?;
    let bookmarks = stmt.query_map([], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            description: row.get(3)?,
        })
    })?;
    for bookmark in bookmarks {
        bookmarks_vec.push(bookmark.unwrap());
    }
    Ok(bookmarks_vec)
}

pub fn remove_bookmark(id: &i32, connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    create_table(connection)?;
    let id_param = Option::from(id);
    let mut stmt = connection.prepare("DELETE FROM bookmarks WHERE id = ?")?;
    stmt.execute([id_param])?;
    Ok(())
}

pub fn get_bookmark(
    id: &i32,
    connection: &rusqlite::Connection,
) -> Result<Bookmark, rusqlite::Error> {
    create_table(connection)?;
    let id_param = Option::from(id);
    let mut stmt = connection.prepare("SELECT * FROM bookmarks WHERE id = ?")?;
    let bookmark = stmt.query_row([id_param], |row| {
        Ok(Bookmark {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            description: row.get(3)?,
        })
    })?;
    Ok(bookmark)
}

pub fn update_bookmark(
    id: &i32,
    new_bookmark: &Bookmark,
    connection: &rusqlite::Connection,
) -> Result<Bookmark, rusqlite::Error> {
    create_table(connection)?;
    // Check if bookmark with given path exists
    let mut stmt = connection.prepare("SELECT * FROM bookmarks WHERE id = ? AND path = ?")?;
    let bookmark = stmt.query_row(
        &[&Option::from(id.to_string()), &new_bookmark.path],
        |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                description: row.get(3)?,
            })
        },
    )?;

    if bookmark.id.is_some() {
        println!("Found bookmark with given path in the database");
        if bookmark.id.unwrap() != *id {
            println!(
                "Bookmark with path {} already exists",
                bookmark.path.unwrap()
            );
            return Err(rusqlite::Error::InvalidQuery);
        }
    }
    let mut stmt = connection
        .prepare("UPDATE bookmarks SET name = ?, path = ?, description = ? WHERE id = ?")?;
    stmt.execute(&[
        &new_bookmark.name,
        &new_bookmark.path,
        &new_bookmark.description,
        &Option::from(id.to_string()),
    ])?;

    Ok(new_bookmark.clone())
}

fn create_table(connection: &rusqlite::Connection) -> Result<(), rusqlite::Error> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            id INTEGER PRIMARY KEY,
            name TEXT,
            path TEXT,
            description TEXT
        )",
        [],
    )?;
    Ok(())
}
