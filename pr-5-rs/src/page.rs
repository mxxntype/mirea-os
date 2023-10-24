use colored::Colorize;
use std::{cell::RefCell, fmt, rc::Rc};

use crate::ram::Ram;

pub const PAGE_DIM: usize = 16;
pub const PAGE_SIZE: usize = PAGE_DIM.pow(2);
pub const PAGE_COUNT: usize = 2;

/// RAM page model. Holds references to bytes owned by RAM.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Page {
    pub bytes: Rc<[RefCell<u8>; PAGE_SIZE]>,
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
        Self { bytes }
    }
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, byte) in self.bytes.iter().enumerate() {
            if i % PAGE_DIM == 0 {
                writeln!(f)?;
            }
            let mut tmp = format!("{:02X}", *byte.borrow()).bold().bright_black();
            if *byte.borrow() != 0 {
                tmp = tmp.blue();
            }
            write!(f, "{tmp} ")?;
        }
        Ok(())
    }
}
