#[derive(Debug)]
pub struct OutOfMemoryError;

impl std::fmt::Display for OutOfMemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OOM")
    }
}

impl std::error::Error for OutOfMemoryError {}

pub struct BumpAllocator {
    bump: usize,
    heap_size: usize,
}

impl BumpAllocator {
    pub fn new() -> Self {
        BumpAllocator {
            bump: 0,
            heap_size: 0,
        }
    }

    pub fn init(capacity: usize) -> Self {
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

    pub fn used(&self) -> usize {
        self.bump
    }
}
