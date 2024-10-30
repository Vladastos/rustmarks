use super::Bookmark;
use skim::prelude::*;

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
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    send_items(bookmarks.to_vec(), tx);
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

fn send_items(items: Vec<Bookmark>, tx: SkimItemSender) {
    for item in items {
        tx.send(Arc::new(item)).unwrap();
    }
}

fn delete_selected_items(items: Vec<Arc<dyn SkimItem>>) -> Vec<Arc<dyn SkimItem>> {
    
    for item in items.clone() {
        let bookmark = (*item).as_any().downcast_ref::<Bookmark>().expect("Something wrong with downcast");
        super::remove_bookmark(&bookmark.id.unwrap());
    }
    items
}
