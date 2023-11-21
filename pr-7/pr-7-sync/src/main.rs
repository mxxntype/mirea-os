use eyre::Result;
use owo_colors::OwoColorize;
use rand::{thread_rng, Rng};
use std::{
    sync::{Arc, Mutex},
    thread,
};

const MIN_CLIENTS: usize = 5;
const MAX_CLIENTS: usize = 5;
const SEAT_COUNT: usize = 3;

#[derive(Debug, Default)]
pub struct Server {}

#[derive(Debug, Default)]
pub struct Client {
    id: usize,
    handled: bool,
}

#[derive(Debug, Default)]
pub struct Seat {
    id: usize,
    taken: bool,
}

impl Server {
    /// Имитация работы
    pub fn handle(&self, client: &mut Client) {
        thread::sleep(std::time::Duration::from_millis(
            thread_rng().gen_range(4..=8),
        ));
        client.handled = true;
        println!("Клиент #{} обслужен и уходит!", client.id.blue());
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Создание парикмахера.
    let server: Arc<Mutex<Server>> = Arc::default();

    // Создание клиентов.
    let client_count: usize = thread_rng().gen_range(MIN_CLIENTS..=MAX_CLIENTS);
    let clients: Vec<Arc<Mutex<Client>>> = (0..client_count)
        .map(|id| Arc::new(Mutex::new(Client { id, handled: false })))
        .collect();

    // Создание мест.
    let seats: Vec<Arc<Mutex<Seat>>> = (0..SEAT_COUNT)
        .map(|id| Arc::new(Mutex::new(Seat { id, taken: false })))
        .collect();

    let mut handles = vec![];

    clients.clone().into_iter().for_each(|client| {
        let server = server.clone();
        let seats = seats.clone();

        let handle = thread::spawn(move || {
            let mut client = client.lock().unwrap();
            let free_seat = seats
                .iter()
                .find(|s| s.try_lock().map_or(false, |seat| !seat.taken));
            match free_seat {
                Some(seat) => {
                    let mut seat = seat.lock().unwrap();
                    seat.taken = true;
                    println!(
                        "Клиент #{} занял место #{}",
                        client.id.green(),
                        seat.id.cyan()
                    );
                    server.lock().unwrap().handle(&mut client);
                    seat.taken = false;
                }
                None => {
                    println!("Клиент #{} не нашел места и уходит.", client.id.red());
                }
            };
        });

        handles.push(handle);
    });

    // Событие синхронизации.
    for handle in handles {
        handle.join().unwrap();
    }

    // [`SEAT_COUNT`] клиентов точно должны были быть обслужены.
    #[cfg(debug_assertions)]
    assert!(
        clients
            .iter()
            .filter(|c| c.try_lock().unwrap().handled)
            .count()
            >= SEAT_COUNT
    );

    Ok(())
}
