#![allow(dead_code)]
#![allow(unused_imports)]
//! The main purpose of this crate is to provide convinient ways to communicate with selenium server
//! and write automated UI tests in Rust.
//! To start using it, you need to have the selenium server running on localhost:4444 and chromedriver or geckodriver
//! be present in your path
mod actions;
mod browser;
mod capabilities;
mod chromeoptions;
mod element;
mod firefoxoptions;
mod proxy;
mod reqs;
mod safarioptions;
mod specialkey;

pub use actions::*;
pub use browser::*;
pub use capabilities::*;
pub use chromeoptions::*;
pub use element::*;
pub use firefoxoptions::*;
pub use proxy::*;
pub use reqs::*;
pub use safarioptions::*;
pub use specialkey::*;
