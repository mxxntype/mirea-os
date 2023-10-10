use color_eyre::Result;
use colored::Colorize;
use std::{
    sync::Arc,
    thread::{self},
    time::Duration,
};
use tokio::sync::Semaphore;

const MAX_PERMIT_COUNT: usize = 4;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(MAX_PERMIT_COUNT));
    let mut point_count: usize = 0;

    create_worker_thread("A".bold().blue(), Arc::clone(&semaphore), 50).await?;

    print_checkpoint(&mut point_count);

    let thread_j = {
        let name = "J".bold().magenta();
        let semaphore = Arc::clone(&semaphore);
        create_worker_thread(name, semaphore, 450)
    };

    let mut join_handles = vec![];

    for i in ["B".bold().red(), "C".bold().yellow(), "I".bold().green()] {
        thread::sleep(Duration::from_millis(25));
        let semaphore = Arc::clone(&semaphore);
        let handle = create_worker_thread(i, semaphore, 75);
        join_handles.push(handle);
    }

    for handle in join_handles {
        handle.await?;
    }

    print_checkpoint(&mut point_count);

    join_handles = vec![];
    for i in ["D".bold().cyan(), "E".bold().blue(), "F".bold().magenta()] {
        thread::sleep(Duration::from_millis(25));
        let semaphore = Arc::clone(&semaphore);
        let handle = create_worker_thread(i, semaphore, 75);
        join_handles.push(handle);
    }

    for handle in join_handles {
        handle.await?;
    }

    print_checkpoint(&mut point_count);

    join_handles = vec![];
    for i in ["G".bold().red(), "H".bold().yellow()] {
        thread::sleep(Duration::from_millis(25));
        let semaphore = Arc::clone(&semaphore);
        let handle = create_worker_thread(i, semaphore, 75);
        join_handles.push(handle);
    }

    for handle in join_handles {
        handle.await?;
    }

    thread_j.await?;

    print_checkpoint(&mut point_count);

    println!("Поток {} начался.", "K".bold().blue());
    println!("\n{}", "--- The End ---".bold().italic());

    Ok(())
}

fn create_worker_thread(
    name: impl Send + 'static + std::fmt::Display,
    semaphore: Arc<Semaphore>,
    lifetime: u64,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        println!("Поток {name} начался.");
        let _permit = semaphore.try_acquire().unwrap();
        println!("Поток {name} захватывает семафор.");
        println!(
            "Переменная семафора равна: {}.",
            MAX_PERMIT_COUNT - semaphore.available_permits()
        );
        thread::sleep(Duration::from_millis(lifetime));
        println!("Поток {name} выходит из семафора.");
    })
}

fn print_checkpoint(count: &mut usize) {
    *count += 1;
    println!("\n{}\n", format!("--- Точка {count} ---").bold().italic());
}
