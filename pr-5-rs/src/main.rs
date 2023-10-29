mod page;
mod process;
mod ram;

use crate::{page::Page, process::Process, ram::Ram};
use color_eyre::Result;
use std::rc::Rc;

fn main() -> Result<()> {
    color_eyre::install()?;

    let ram = Rc::new(Ram::new());
    let mut page = Page::new(0, &ram);

    for _ in 0..5 {
        let process = Process::new();
        let pid = process.pid;
        println!("Loading process with PID {pid}...");
        page.load_proccess(&process)?;
        page.unload_process(pid)?;
        println!("{process}");
    }

    Ok(())
}
