use super::Bookmark;
use skim::prelude::*;

pub fn open_selection_dialog(bookmarks: &Vec<Bookmark>) -> Result<String, String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .preview(Some("inline"))
        .layout("reverse")
        .build()
        .unwrap();
    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    send_items(bookmarks.to_vec(), tx);
    let selected_bookmark = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let result = selected_bookmark
        .get(0)
        .map(|selected_bookmark| selected_bookmark.output().into_owned());

    //FIX: Returning selection on user exit
    return Ok(result.unwrap_or_default());
}

fn send_items(items: Vec<Bookmark>, tx: SkimItemSender) {
    for item in items {
        tx.send(Arc::new(item)).unwrap();
    }
}
