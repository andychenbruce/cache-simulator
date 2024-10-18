fn main() {
    let mut cache = Cache::new(4, 8, 8);

    for addr in [29, 26, 45, 61, 29, 58, 232, 125, 29, 61] {
        cache.access(addr);
    }
}

struct Cache {
    index_bits: u64,
    addr_bits: u64,
    offset_bits: u64,
    state: Vec<Option<u64>>,
}

impl Cache {
    fn new(num_entries: u64, entry_size_bytes: u64, addr_bits: u64) -> Self {
        Cache {
            index_bits: get_log_2(num_entries),
            addr_bits,
            offset_bits: get_log_2(entry_size_bytes),
            state: vec![None; usize::try_from(num_entries).unwrap()],
        }
    }

    fn access(&mut self, addr: u64) {
        assert!(get_largest_bit_pos(addr) <= self.addr_bits);

        let tag_bits = self.addr_bits - self.index_bits - self.offset_bits;

        let offset_mask: u64 = (1 << self.offset_bits) - 1;
        let index_mask: u64 = ((1 << (self.index_bits + self.offset_bits)) - 1) ^ offset_mask;
        let tag_mask: u64 =
            ((1 << (tag_bits + self.index_bits + self.offset_bits)) - 1) ^ index_mask ^ offset_mask;

        let offset = addr & offset_mask;
        let index = (addr & index_mask) >> self.offset_bits;
        let tag = (addr & tag_mask) >> (self.offset_bits + self.index_bits);

        assert!(get_largest_bit_pos(offset) <= self.offset_bits);
        assert!(get_largest_bit_pos(index) <= self.index_bits);
        assert!(get_largest_bit_pos(tag) <= tag_bits);

        print!("addr = ");
        let string = format!("{:010b}", addr);
        for (pos, c) in string.chars().enumerate() {
            let i: u64 = (10 - pos - 1).try_into().unwrap();
            if (i == self.offset_bits) || (i == self.offset_bits + self.index_bits) {
                print!("{}|", c);
            } else {
                print!("{}", c);
            }
        }
        println!();
        println!("tag = {}, index = {}, offset = {}", tag, index, offset);

        let index: usize = index.try_into().unwrap();
        if let Some(entry_tag) = self.state[index] {
            if entry_tag == tag {
                println!("hit");
            } else {
                println!("miss, wrong tag");
                self.state[index] = Some(tag);
            }
        } else {
            println!("miss, invaid");
            self.state[index] = Some(tag);
        }
    }
}

fn get_log_2(x: u64) -> u64 {
    assert!(x.count_ones() == 1);
    x.ilog2().into()
}

fn get_largest_bit_pos(x: u64) -> u64 {
    (u64::BITS - x.leading_zeros()).into()
}
