use thiserror::Error;

#[derive(Error, Debug)]
#[error("OOM")]
pub struct OutOfMemoryError;

struct BumpAllocator {
    bump: usize,
    heap_size: usize,
}

impl BumpAllocator {
    pub fn new(capacity: usize) -> Self {
        BumpAllocator {
            bump: 0,
            heap_size: capacity,
        }
    }

    pub fn alloc(&mut self, size_of_alloc: usize) -> Result<usize, OutOfMemoryError> {
        if self.bump + size_of_alloc > self.heap_size {
            return Err(OutOfMemoryError);
        }
        let address_start = self.bump;
        self.bump += size_of_alloc;
        Ok(address_start)
    }

    pub fn reset(&mut self) {
        self.bump = 0;
    }

    pub fn used(self) -> usize {
        self.bump
    }
}
