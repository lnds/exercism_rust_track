pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    head: usize,
    tail: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
where
    T: Clone + Default + std::fmt::Debug,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![Default::default(); capacity + 1],
            head: 0,
            tail: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        let next = (self.tail + 1) % self.buffer.capacity();
        if next == self.head {
            return Err(Error::FullBuffer);
        }
        self.buffer[self.tail] = element;
        self.tail = next;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.head == self.tail {
            return Err(Error::EmptyBuffer);
        }
        let next = (self.head + 1) % self.buffer.capacity();
        let value = self.buffer[self.head].clone();
        self.head = next;
        Ok(value)
    }

    pub fn clear(&mut self) {
        self.head = self.tail;
    }

    #[allow(unused_must_use)] 
    pub fn overwrite(&mut self, element: T) {
        let next = (self.tail + 1) % self.buffer.capacity();
        if next == self.head {
            self.head = (self.head + 1) % self.buffer.capacity();
        } 
        self.write(element);
    }
}
