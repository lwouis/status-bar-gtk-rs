# Purpose

A status bar based on [gtk-rs](http://gtk-rs.org/). Goal is to provide a bar with indicators that are not only 
text-based (such as [i3status](https://i3wm.org/i3status/) or [i3status-rust](https://github.com/greshake/i3status-rust))

# How to install

* If you don't have it already, install [Rust](https://www.rust-lang.org)
* [gtk-rs](http://gtk-rs.org/) [has requirements](http://gtk-rs.org/docs/requirements.html) you 
need to fulfill
* `git clone` this repo
* Run `cargo build --release`

# How to use

* Add this line to `.i3/config` file: `exec_always --no-startup-id [PATH_TO_BINARY]`
* _TODO: Find a way to integrate with i3_ (maybe see [how Polybar does it](https://github.com/jaagr/polybar/wiki))