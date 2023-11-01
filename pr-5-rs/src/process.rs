use colored::Colorize;
use rand::{self, Rng};
use std::fmt;

pub const PROCESS_SIZE: usize = 32;

#[derive(Debug)]
pub struct Process {
    pub(crate) pid: u16,
    pub(crate) instructions: [u8; PROCESS_SIZE],
}

impl Process {
    /// Generate a [`Process`] with a random `pid` and random `instructions`.
    #[must_use]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            pid: rng.gen(),
            instructions: rng.gen(),
        }
    }

    /// Generate a [`Process`] with a set `pid` and random `instructions`.
    #[must_use]
    pub fn with_pid(pid: u16) -> Self {
        let mut process = Self::new();
        process.pid = pid;
        process
    }
}

impl fmt::Display for Process {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        write!(
            f,
            "\t┌── Процесс с PID {:5} ──────────────────────────┐",
            self.pid
        )?;
        for (i, byte) in self.instructions.iter().enumerate() {
            if i % (PROCESS_SIZE / 2) == 0 {
                write!(f, "\n\t│ ")?;
            }
            let formatted_byte = format!("{byte:02x}").red();
            write!(f, "{formatted_byte} ")?;
            if i % (PROCESS_SIZE / 2) == (PROCESS_SIZE / 2) - 1 {
                write!(f, "│")?;
            }
        }
        writeln!(f, "\n\t└─────────────────────────────────────────────────┘")?;
        Ok(())
    }
}
