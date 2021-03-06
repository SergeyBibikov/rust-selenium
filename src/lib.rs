#![allow(dead_code)]
#![allow(unused_imports)]
//! The main purpose of this crate is to provide convinient ways to communicate with selenium server
//! and write automated UI tests in Rust.
//! To start using it, you need to have the selenium server running on localhost:4444 and chromedriver or geckodriver
//! be present in your path
mod reqs;
mod browser;
mod element;
mod actions;
mod specialkey;
mod chromeoptions;
mod firefoxoptions;
mod safarioptions;
mod proxy;
mod capabilities;

pub use browser::*;
pub use actions::*;
pub use element::*;
pub use specialkey::*;
pub use reqs::*;
pub use chromeoptions::*;
pub use firefoxoptions::*;
pub use safarioptions::*;
pub use proxy::*;
pub use capabilities::*;