mod file;
mod page;
mod process;
mod ram;

use crate::{
    page::{Page, PAGE_SIZE},
    process::{Process, PROCESS_SIZE},
    ram::Ram,
};
use color_eyre::{owo_colors::OwoColorize, Result};
use file::{File, Filesystem};
use page::MAX_PAGE_COUNT;
use rand::Rng;
use std::{fmt::Display, rc::Rc};

fn main() -> Result<()> {
    color_eyre::install()?;

    #[cfg(feature = "ram")]
    {
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
    }

    #[cfg(feature = "fs")]
    {
        const DISPLAY_BLOCK_COUNT: usize = 2;

        status_message(&"Создание файловой системы...");
        let mut fs = Filesystem::default();

        status_message(&"Создание файла...");
        let mut file = File::default();

        status_message(&"Переименование файла в main.rs...");
        file.rename(&"main.rs");

        status_message(&format!(
            "Резервирование {DISPLAY_BLOCK_COUNT} блоков для файла {}...",
            file.name
        ));
        file.reserve(DISPLAY_BLOCK_COUNT);

        file.show_blocks();
        file.reserve(64 * 1024 / 512 - DISPLAY_BLOCK_COUNT);
        fs.add_file(&file);

        status_message(&format!("Резервирование 64КБ для файла {}...", file.name));
        status_message(&format!("Файловая система использует {} байт.", fs.usage()));
    }

    Ok(())
}

fn status_message(msg: &(impl Display + Clone)) {
    println!("\t\t{}", msg.clone().italic());
}
