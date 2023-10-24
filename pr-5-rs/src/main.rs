use color_eyre::Result;
use std::rc::Rc;

mod page;
mod process;
mod ram;

use crate::{page::Page, process::Process, ram::Ram};

fn main() -> Result<()> {
    color_eyre::install()?;

    let ram = Rc::new(Ram::new());
    let page = Page::new(0, &ram);

    for i in 0..page::PAGE_SIZE / process::PROCESS_SIZE {
        let process = Process::new();
        process.write_to_page(&page, i);
    }

    println!("{page}");

    Ok(())
}
