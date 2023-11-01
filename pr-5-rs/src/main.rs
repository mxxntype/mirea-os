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

    let ram = Rc::new(Ram::default());
    let mut pages: Vec<Page> = vec![];
    for i in 0..MAX_PAGE_COUNT {
        pages.push(Page::new(i, &ram));
        pages[i].id = i + 1;

        let mut rng = rand::thread_rng();
        let process_count: usize = rng.gen_range(1..PAGE_SIZE / PROCESS_SIZE);
        println!(
            "\t\tЗагрузка {process_count} процессов в RAM на страницу №{}...",
            pages[i].id
        );
        for _ in 0..process_count {
            let process = Process::with_pid(rng.gen());
            println!("{process}");
            pages[i].load_process(&process)?;
            println!(
                "\t\tПроцессов в оперативной памяти: {}",
                pages.iter().map(|p| p.loaded_processes).sum::<usize>()
            );
        }

        println!("{}", pages[i]);
    }

    Ok(())
}
