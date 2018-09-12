
use std::hash::Hasher;

pub struct ValueHasher<'a>(pub &'a mut Hasher);

impl<'a> Hasher for ValueHasher<'a> {
    fn finish(&self) -> u64 {
        self.0.finish()
    }
    fn write(&mut self, data: &[u8]) {
        self.0.write(data)
    }
}


