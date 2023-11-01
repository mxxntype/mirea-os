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

impl Page {
    /// Load a [`Process`] into memory.
    #[allow(dead_code)]
    pub fn load_process(&mut self, process: &Process) -> Result<()> {
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
        let offset = self.map.remove(&pid).ok_or(eyre::eyre!(
            "Cannot unload process: process with pid {pid} isn't loaded"
        ))?;
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
        writeln!(f)?;
        write!(f, "{}", format!("\tProcess PID {}:", self.pid).bold())?;
        for (i, byte) in self.instructions.iter().enumerate() {
            if i % (PROCESS_SIZE / 2) == 0 {
                writeln!(f)?;
            }
            let formatted_byte = format!("{byte:02x}").red();
            write!(f, "{formatted_byte} ")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Process;
    use crate::{page::Page, ram::Ram};
    use color_eyre::Result;
    use std::rc::Rc;

    #[test]
    fn load_unload_match() -> Result<()> {
        const PID2: u16 = 123;
        let ram = Rc::new(Ram::new());
        let mut page = Page::new(0, &ram);
        let first_process = Process::new();
        let second_process = Process::with_pid(PID2);
        page.load_process(&first_process)?;
        page.load_process(&second_process)?;
        assert_eq!(
            first_process.pid,
            page.unload_process(first_process.pid)?.pid
        );
        assert_eq!(PID2, page.unload_process(PID2)?.pid);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn incorrect_unload() {
        let ram = Rc::new(Ram::new());
        let mut page = Page::new(0, &ram);
        let _ = page.unload_process(321).unwrap();
    }
}
