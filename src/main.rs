#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod core;
mod constants;
mod structures;

mod app;

use crate::app::App;

fn main() -> anyhow::Result<()> {
    App::run()
}

use core::*;