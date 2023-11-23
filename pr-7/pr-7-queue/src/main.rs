use eyre::Result;
use rand::{Rng, RngCore};
use std::collections::VecDeque;

fn main() -> Result<()> {
    color_eyre::install()?;

    let element_count: usize = rand::thread_rng().gen_range(8..=16);

    println!("Создание пустой очереди...");
    let mut queue: Queue<Element> = Queue::new();

    println!("Добавление {element_count} елементов в очередь...");
    (0..element_count).for_each(|_| queue.enqueue(Element::default()));

    println!("Обмен местами первого и последнего элемента в очереди...");
    queue.swap_ends();

    println!("Опустошение очереди...");
    while queue.dequeue().is_some() {}

    assert!(queue.is_empty());

    Ok(())
}

#[derive(Debug, Default)]
pub struct Queue<T> {
    elements: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Creates and empty queue.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            elements: VecDeque::new(),
        }
    }

    /// Appends an element to the back of the queue.
    pub fn enqueue(&mut self, item: T) {
        self.elements.push_back(item);
    }

    /// Removes the first element and returns it, or None if the queue is empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop_front()
    }

    /// Returns `true` if the queue is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get an immutable reference to the first element in the queue.
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.elements.iter().last()
    }

    /// Get a mutable reference to the first element in the queue.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.elements.iter_mut().last()
    }

    /// Swap the first and the last element of the queue.
    pub fn swap_ends(&mut self) {
        if self.elements.len() >= 2 {
            let initial_head = self.dequeue().unwrap();
            let mut tmp = Self::new();
            while self.elements.len() > 1 {
                tmp.enqueue(self.dequeue().unwrap());
            }
            while !tmp.is_empty() {
                self.enqueue(tmp.dequeue().unwrap());
            }
            self.enqueue(initial_head);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element {
    /// 20KB of data required by the task.
    data: [u8; 20 * 1024],
}

impl Default for Element {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let mut data = [0; 20 * 1024];
        rng.fill_bytes(&mut data);
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Element, Queue};

    /// Check if an empty queue is created correctly.
    #[test]
    pub fn create_empty() {
        let mut queue: Queue<Element> = Queue::new();
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
    }

    /// Check if `enqueue()` and `dequeue()` work as expected.
    #[test]
    fn enqueue_and_dequeue() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert!(!queue.is_empty());

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.dequeue(), None);

        assert!(queue.is_empty());
    }

    /// Check that the `swap_end` function works correctly.
    #[test]
    fn swap() {
        let mut q: Queue<i32> = Queue::new();

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        q.enqueue(4);

        q.swap_ends();

        assert_eq!(q.dequeue(), Some(4));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert_eq!(q.dequeue(), Some(1));
    }
}
