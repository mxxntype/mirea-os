use colored::Colorize;
use rand::Rng;
use std::{collections::HashMap, fmt};

pub const BLOCK_SIZE: usize = 512;
pub const BLOCK_DIM: usize = BLOCK_SIZE / 16;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct Filesystem {
    pub(crate) files: HashMap<String, File>,
}

#[allow(dead_code)]
impl Filesystem {
    pub fn add_file(&mut self, file: &File) {
        self.files.insert(file.name.clone(), file.clone());
    }

    pub fn get_files(&self) -> Vec<File> {
        self.files.values().cloned().collect()
    }

    pub fn usage(&self) -> usize {
        self.files
            .values()
            .map(|f| f.blocks.len() * BLOCK_SIZE)
            .sum()
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct File {
    pub(crate) owner: Owner,
    pub(crate) name: String,
    pub(crate) mode: u16,
    pub(crate) blocks: Vec<Block>,
}

#[allow(dead_code)]
impl File {
    /// `drop` the [`File`].
    fn delete(self) {
        drop(self);
    }

    /// Change the [`File`]'s `name`.
    pub fn rename(&mut self, new_name: &(impl Into<String> + Clone)) {
        self.name = new_name.clone().into();
    }

    /// Change the [`File`]'s `mode`.
    pub fn chmod(&mut self, new_mode: &(impl Into<u16> + Clone)) {
        self.mode = new_mode.clone().into();
    }

    /// Change the [`File`]'s `owner`.
    pub fn chown(&mut self, new_owner: &(impl Into<Owner> + Clone)) {
        self.owner = new_owner.clone().into();
    }

    pub fn reserve(&mut self, block_count: usize) {
        (0..block_count).for_each(|_| self.blocks.push(Block::default()));
    }

    pub fn show_blocks(&self) {
        println!("\n\t\tБлоки файла '{}':\n", self.name.bold().purple());
        self.blocks.iter().for_each(|b| println!("{b}"));
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Block {
    bytes: [u8; BLOCK_SIZE],
}

impl Default for Block {
    fn default() -> Self {
        let mut bytes = [0u8; BLOCK_SIZE];
        rand::thread_rng().fill(&mut bytes[..]);
        Self { bytes }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let size = BLOCK_DIM;
        writeln!(f, "\t┌── Block {:─<1$}┐", "", size * 3 - 8)?;
        for chunk in self.bytes.chunks(size) {
            write!(f, "\t│ ")?;
            for byte in chunk {
                let formatted_byte = format!("{byte:02x}").green();
                write!(f, "{formatted_byte} ")?;
            }
            writeln!(f, "│")?;
        }
        writeln!(f, "\t└{:─<1$}┘", "", size * 3 + 1)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Owner {
    pub(crate) name: String,
    pub(crate) group: String,
}

impl Default for Owner {
    fn default() -> Self {
        Self {
            name: String::from("user"),
            group: String::from("group"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{File, Owner};

    #[test]
    fn rename() {
        let mut file = File::default();

        let new_filename: &str = "123";
        file.rename(&new_filename);
        assert_eq!(file.name, new_filename);

        file.delete();
    }

    #[test]
    fn chmod() {
        let mut file = File::default();

        let new_mode = 0b0111_1011;
        file.chmod(&new_mode);
        assert_eq!(new_mode, file.mode);

        file.delete();
    }

    #[test]
    fn chown() {
        let mut file = File::default();

        let new_owner = Owner {
            name: String::from("newname"),
            group: String::from("newgroup"),
        };
        file.chown(&new_owner);
        assert_eq!(new_owner, file.owner);

        file.delete();
    }
}
