use colored::Colorize;
use std::{cell::RefCell, fmt, rc::Rc};

const PAGE_DIM: usize = 16;
const PAGE_SIZE: usize = PAGE_DIM.pow(2);
const PAGE_COUNT: usize = 2;
const RAM_SIZE: usize = PAGE_SIZE * PAGE_COUNT;

/// RAM model. Owns the bytes.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Ram {
    pub bytes: RefCell<[u8; RAM_SIZE]>,
}

impl Ram {
    /// Creates a new RAM object.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: RefCell::new([0; RAM_SIZE]),
        }
    }
}

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
