use color_eyre::Result;
use std::rc::Rc;

use crate::{
    process::Process,
    ram::{Page, Ram},
};

mod process;
mod ram;
mod swap;

fn main() -> Result<()> {
    color_eyre::install()?;

    let ram = Rc::new(Ram::new());
    let page_0 = Page::new(0, &ram);

    let process = Process::new();
    process.write_to_page(&page_0);

    println!("{page_0}");

    Ok(())
}
