use gtk::{Popover, SearchBar};
use gtk::prelude::{PopoverExt, WidgetExt};

pub(crate) fn make_find() -> Popover {
    let search_bar = SearchBar::builder().build();
    search_bar.set_size_request(500, 50);
    let popover = Popover::builder().build();
    popover.set_child(Some(&search_bar));
    popover
}