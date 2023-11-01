mod page;
mod process;
mod ram;

use crate::{
    page::{Page, PAGE_SIZE},
    process::{Process, PROCESS_SIZE},
    ram::Ram,
};
use color_eyre::Result;
use page::MAX_PAGE_COUNT;
use rand::Rng;
use std::rc::Rc;

fn main() -> Result<()> {
    color_eyre::install()?;

    let ram = Rc::new(Ram::new());
    let mut pages: Vec<Page> = vec![];
    for i in 0..MAX_PAGE_COUNT {
        pages.push(Page::new(i, &ram));

        let mut rng = rand::thread_rng();
        let process_count: usize = rng.gen_range(0..PAGE_SIZE / PROCESS_SIZE);
        println!("Loading {process_count} processes into RAM...");
        for _ in 0..process_count {
            let process = Process::with_pid(rng.gen());
            println!("{process}");
            pages[i].load_process(&process)?;
        }

        println!("{}", pages[i]);
    }

    Ok(())
}
