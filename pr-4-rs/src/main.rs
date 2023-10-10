use color_eyre::Result;
use colored::Colorize;
use std::{sync::Arc, thread, time::Duration};
use tokio::sync::Semaphore;

/// Максимальное количество одновременных доступов к семафоре
const MAX_PERMIT_COUNT: usize = 4;
const THREAD_SPAWN_INTERVAL: u64 = 25;
const THREAD_LIFETIME: u64 = 75;

/// Вариант #2
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let semaphore: Arc<Semaphore> = Arc::new(Semaphore::new(MAX_PERMIT_COUNT));
    let mut point_count: usize = 0;

    // Создание и присоединения потока A
    create_worker_thread("A".bold().blue(), Arc::clone(&semaphore), THREAD_LIFETIME).await?;

    print_current_point(&mut point_count);

    // Создание потока J
    let thread_j = create_worker_thread(
        "J".bold().magenta(),
        Arc::clone(&semaphore),
        9 * THREAD_SPAWN_INTERVAL + 3 * THREAD_LIFETIME,
    );

    // Создание и присоединения потоков B, C, I
    batch_threads(
        vec!["B".bold().red(), "C".bold().yellow(), "I".bold().green()],
        &semaphore,
        THREAD_SPAWN_INTERVAL,
        THREAD_LIFETIME,
    )
    .await?;
    print_current_point(&mut point_count);

    // Создание и присоединения потоков D, E, F
    batch_threads(
        vec!["D".bold().blue(), "E".bold().cyan(), "F".bold().magenta()],
        &semaphore,
        THREAD_SPAWN_INTERVAL,
        THREAD_LIFETIME,
    )
    .await?;
    print_current_point(&mut point_count);

    // Создание и присоединения потоков G, H
    batch_threads(
        vec!["G".bold().red(), "H".bold().yellow()],
        &semaphore,
        THREAD_SPAWN_INTERVAL,
        THREAD_LIFETIME,
    )
    .await?;

    // Присоединение потока J
    thread_j.await?;

    print_current_point(&mut point_count);

    println!("Поток {} начался.", "K".bold().blue());
    println!("\n{}", "--- The End ---".bold().italic());

    Ok(())
}

/// Создать и соединить именованный поток с семафорой
// Без временной задержки потоки завершаются моментально,
// из-за чего информационные сообщения выводятся на экран
// в непредсказуемом порядке.
fn create_worker_thread(
    name: impl Send + 'static + std::fmt::Display,
    semaphore: Arc<Semaphore>,
    lifetime: u64,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        println!("Поток {name} начался и ожидает семафор.");
        println!("Поток {name} захватывает семафор.");
        let _permit = semaphore.try_acquire().unwrap();
        println!("Поток {name} в семафоре.");
        println!(
            "Переменная семафора равна: {}.",
            MAX_PERMIT_COUNT - semaphore.available_permits()
        );
        thread::sleep(Duration::from_millis(lifetime));
        println!("Поток {name} выходит из семафора.");
    })
}

async fn batch_threads(
    thread_names: Vec<impl Sync + Send + 'static + std::fmt::Display>,
    semaphore: &Arc<Semaphore>,
    spawn_interval: u64,
    thread_lifetime: u64,
) -> Result<()> {
    let mut join_handles = vec![];
    for i in thread_names {
        thread::sleep(Duration::from_millis(spawn_interval));
        let semaphore = Arc::clone(semaphore);
        let handle = create_worker_thread(i, semaphore, thread_lifetime);
        join_handles.push(handle);
    }

    // Присоединение потоков B, C, I
    for handle in join_handles {
        handle.await?;
    }
    Ok(())
}

/// Визуализация точек
fn print_current_point(point_count: &mut usize) {
    *point_count += 1;
    println!(
        "\n{}\n",
        format!("--- Точка {point_count} ---").bold().italic()
    );
}
