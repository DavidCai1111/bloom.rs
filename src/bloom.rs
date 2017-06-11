use bit_vec::BitVec;
use std::hash::Hash;

#[derive(Debug)]
pub struct BloomFilter {
    bit_vec: BitVec,
    times: usize,
}

impl BloomFilter {
    /// Creates a new BloomFilter instance.
    pub fn new(bits_num: usize, times: usize) -> Self {
        BloomFilter {
            bit_vec: BitVec::with_capacity(bits_num),
            times,
        }
    }

    /// Returns the capacity of the bloom filter.
    pub fn bits_num(&self) -> usize {
        self.bit_vec.capacity()
    }

    /// Add a new item to the bloom filter.
    pub fn add<T: Hash>(&mut self, data: T) -> &mut Self {
        let pos = self.get_pos(data);
        self.bit_vec.set(pos, true);
        self
    }

    /// Check whether the item is in the bloom filter. If true, the result
    /// might be false positive. If false, the data is definitely not in
    /// the set.
    pub fn contains<T: Hash>(&self, data: T) -> bool {
        match self.bit_vec.get(self.get_pos(data)) {
            Some(result) => result,
            None => false,
        }
    }

    /// Clear all items inserted in the bloom filter.
    pub fn clear_all(&mut self) -> &mut Self{
        self.bit_vec.clear();
        self
    }

    fn get_pos<T: Hash>(&self, data: T) -> usize {
        unimplemented!()
    }
}
