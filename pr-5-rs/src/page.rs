use crate::PROCESS_SIZE;
use crate::{process::Process, ram::Ram};
use color_eyre::Result;
use colored::Colorize;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

pub const PAGE_DIM: usize = 16;
pub const PAGE_SIZE: usize = PAGE_DIM.pow(2);
pub const MAX_PAGE_COUNT: usize = 2;

/// RAM page model. Holds references to bytes owned by [`Ram`].
#[allow(unused)]
#[derive(Debug)]
pub struct Page {
    pub(crate) bytes: Rc<[RefCell<u8>; PAGE_SIZE]>,
    pub(crate) loaded_processes: usize,
    pub(crate) map: HashMap<u16, usize>,
}

impl Page {
    /// Creates a new RAM page.
    #[must_use]
    pub fn new(index: usize, ram: &Rc<Ram>) -> Self {
        let bytes = ram.bytes.borrow()[index..index + PAGE_SIZE]
            .iter()
            .copied()
            .map(RefCell::new)
            .collect::<Vec<_>>();
        let bytes = Rc::new(bytes.try_into().unwrap());
        Self {
            bytes,
            loaded_processes: 0,
            map: HashMap::new(),
        }
    }

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

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "- RAM Page")?;
        for (i, byte) in self.bytes.iter().enumerate() {
            if i % PAGE_DIM == 0 {
                writeln!(f)?;
            }
            let mut tmp = format!("{:02x}", *byte.borrow()).bold().bright_black();
            if *byte.borrow() != 0 {
                tmp = tmp.blue();
            }
            write!(f, "{tmp} ")?;
        }
        Ok(())
    }
}

impl Page {}

#[cfg(test)]
mod tests {
    use crate::Process;
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
