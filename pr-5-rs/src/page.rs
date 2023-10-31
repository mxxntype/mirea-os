use crate::ram::Ram;
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
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "- RAM Page")?;
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
