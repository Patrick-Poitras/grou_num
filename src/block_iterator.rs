use super::grou::{Grou, GrouSubset};

pub struct BlockIterator<'iter> {
    number: &'iter Grou,
    block_length: usize,
    current_index: usize,
}

impl<'iter> BlockIterator<'iter> {
    pub fn new(number: &'iter Grou, block_length: usize) -> BlockIterator {
        BlockIterator {
            number,
            block_length, 
            current_index: 0
        }
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.block_length
    }
}

impl<'iter> Iterator for BlockIterator<'iter> {
    type Item = GrouSubset<'iter>;

    fn next(&mut self) -> Option<GrouSubset<'iter>> {
        if self.block_length == 0 {
            None
        } else {
            let ret_val = Some(self.number.split_off_block(self.current_index, self.block_length));
            if self.current_index + self.block_length >= self.number.len() {
                // Indicates termination. All further elements will return None.
                self.block_length = 0; 
            }

            ret_val
        }
    }
}

impl<'iter> BlockIterator<'iter> {
    pub fn unwrap_next(&mut self) -> GrouSubset<'iter> {
        self.next().unwrap()
    }
}

pub fn calculate_block_length(nblocks:usize, g1: &Grou, g2:&Grou) -> usize {
    let max_length = std::cmp::max(g1.len(), g2.len());

    // Integer division + 1 if the division had a remainder, is equivalent to
    // ceil(max_length / nblocks);
    max_length / nblocks + ((max_length % nblocks != 0) as usize)
}