use rand::{self, Rng};

use crate::ram::Page;

const PROCESS_SIZE: usize = 32;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Process {
    pid: u16,
    instructions: [u8; PROCESS_SIZE],
}

impl Process {
    /// Generate a Process with a random PID and random instructions.
    #[allow(dead_code)]
    #[must_use]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            pid: rng.gen(),
            instructions: rng.gen(),
        }
    }

    pub fn write_to_page(self, page: &Page) {
        for i in 0..PROCESS_SIZE {
            page.bytes[i].try_borrow_mut().map_or_else(
                |_| eprintln!(".borrow_mut() failed on byte {i}"),
                |mut byte_addr| *byte_addr = self.instructions[i],
            );
        }
    }
}
