#[macro_use]
extern crate log;

extern crate env_logger;
extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType, MenuButton, Popover, Label};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<Error>>;

pub fn main() {
    env_logger::init().expect("Log library failed to initialize");
    status_bar().unwrap_or_else(|err| {
        error!("{}", err);
    });
}

fn status_bar() -> Result<()> {
    gtk::init().map_err(|_| "Failed to initialize GTK")?;

    let window = Window::new(WindowType::Toplevel);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let menu_button = MenuButton::new();
    let menu_button_label = Label::new("Sound");
    menu_button.add(&menu_button_label);

    let popover_menu = Popover::new(&menu_button);
    let popover_menu_label = Label::new("Volume: 95%");
    popover_menu.add(&popover_menu_label);

    menu_button.set_popover(&popover_menu);

    window.add(&menu_button);
    window.show_all();
    gtk::main();
    Ok(())
}
