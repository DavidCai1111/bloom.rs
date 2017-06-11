use bit_vec::BitVec;
use std::hash::{Hash, Hasher};
use fnv::FnvHasher;

#[derive(Debug)]
pub struct BloomFilter {
    bit_vec: BitVec,
    bits_num: usize,
    times: usize,
}

type HashPairs = [u64; 4];

impl BloomFilter {
    /// Creates a new BloomFilter instance.
    pub fn new(bits_num: usize, times: usize) -> Self {
        let cap = if bits_num != 0 { bits_num } else { 1 };
        BloomFilter {
            bit_vec: BitVec::from_elem(cap, false),
            bits_num: cap,
            times,
        }
    }

    /// Returns the capacity of the bloom filter.
    pub fn bits_num(&self) -> usize {
        self.bits_num
    }

    /// Add a new item to the bloom filter.
    pub fn add<T: Hash>(&mut self, data: T) {
        let pairs = Self::base_hashes(data);

        for i in 0..self.times {
            self.bit_vec
                .set((Self::get_pos_with_i(pairs, i) % self.bits_num as u64) as usize,
                     true);
        }
    }

    /// Check whether the item is in the bloom filter. If true, the result
    /// might be false positive. If false, the data is definitely not in
    /// the set.
    pub fn contains<T: Hash>(&self, data: T) -> bool {
        let pairs = Self::base_hashes(data);

        for i in 0..self.times {
            match self.bit_vec
                      .get((Self::get_pos_with_i(pairs, i) % self.bits_num as u64) as usize) {
                Some(result) => {
                    if !result {
                        return false;
                    }
                }
                None => unreachable!(),
            }
        }

        true
    }

    /// Clear all items inserted in the bloom filter.
    pub fn clear_all(&mut self) -> &mut Self {
        self.bit_vec.clear();
        self
    }

    fn get_pos_with_i(pairs: HashPairs, i: usize) -> u64 {
        (i as u64 + pairs[2 + (((i + (i % 2)) % 4) / 2)])
    }

    fn base_hashes<T: Hash>(data: T) -> HashPairs {
        let mut result: HashPairs = [0; 4];
        let mut hasher = FnvHasher::default();
        data.hash(&mut hasher);
        result[0] = hasher.finish();
        hasher.write_i32(1);
        result[1] = hasher.finish();
        hasher.write_i32(2);
        result[2] = hasher.finish();
        hasher.write_i32(3);
        result[3] = hasher.finish();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let filter = BloomFilter::new(10, 5);
        assert_eq!(filter.bits_num, 10);
        assert_eq!(filter.times, 5);
        assert!(filter.bit_vec.capacity() >= 10);
    }

    #[test]
    fn test_bits_num() {
        let filter = BloomFilter::new(10, 5);
        assert_eq!(filter.bits_num(), 10);
    }

    #[test]
    fn test_add_and_contains() {
        let mut filter = BloomFilter::new(1000, 5);
        filter.add("test1");
        filter.add("test2");

        assert!(filter.contains("test1"));
        assert!(filter.contains("test2"));
        assert!(!filter.contains("test3"));
    }
}

