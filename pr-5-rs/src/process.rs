use crate::page::Page;
use color_eyre::Result;
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
    /// Generate a Process with a random PID and random instructions.
    #[must_use]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            pid: rng.gen(),
            instructions: rng.gen(),
        }
    }

    #[must_use]
    pub fn with_pid(pid: u16) -> Self {
        let mut process = Self::new();
        process.pid = pid;
        process
    }
}

impl Page {
    /// Load a [`Process`] into memory.
    #[allow(dead_code)]
    pub fn load_proccess(&mut self, process: &Process) -> Result<()> {
        eyre::ensure!(
            self.loaded_processes < crate::page::PAGE_SIZE / PROCESS_SIZE,
            "Not enough space in page to write another process"
        );
        let start: usize = self.loaded_processes * PROCESS_SIZE;
        for i in 0..PROCESS_SIZE {
            let offset = i + start;
            *self.bytes[offset].try_borrow_mut()? = process.instructions[i];
        }
        self.map.insert(process.pid, start);
        self.loaded_processes += 1;
        Ok(())
    }

    /// Unload a [`Process`] from memory.
    #[allow(dead_code)]
    pub fn unload_process(&mut self, pid: u16) -> Result<Process> {
        let offset = self
            .map
            .remove(&pid)
            .expect("No process with such PID: {pid}");
        eyre::ensure!(
            offset + PROCESS_SIZE <= (self.loaded_processes + 1) * PROCESS_SIZE,
            "Cannot unload process from memory[{offset}-{}] not mapped to any process, only bytes [0-{}] are mapped",
            offset + PROCESS_SIZE, (self.loaded_processes + 1) * PROCESS_SIZE
        );
        let mut process = Process::with_pid(pid);
        for i in 0..PROCESS_SIZE {
            process.instructions[i] = *self.bytes[offset].try_borrow()?;
            *self.bytes[offset].try_borrow_mut()? = 0; // Zero out the memory
        }
        self.loaded_processes -= 1;
        Ok(process)
    }
}

impl fmt::Display for Process {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- Process PID: {}", self.pid)?;
        for (i, byte) in self.instructions.iter().enumerate() {
            if i % (PROCESS_SIZE / 2) == 0 {
                writeln!(f)?;
            }
            let formatted_byte = format!("{byte:02X}").red();
            write!(f, "{formatted_byte} ")?;
        }
        Ok(())
    }
}
