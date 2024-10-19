fn main() {
    let mut cache: Cache<4, 1, 8, 8> = Cache::new();

    for addr in [29, 26, 45, 61, 29, 58, 232, 125, 29, 61] {
        cache.access(Addr(addr));
    }

    for (entry, bucket) in cache.state.iter().enumerate() {
        if let Some(val) = bucket {
            println!("entry {}: {}", entry, val.last_accessed_by.0);
        } else {
            println!("entry {}: INVALID", entry);
        }
    }
}

#[derive(Copy, Clone)]
struct Bucket<
    const NUM_ENTRIES: u64,
    const ASSOCIATIVITY: u64,
    const ENTRY_SIZE: u64,
    const ADDR_BITS: u32,
> {
    tag: u64,
    last_accessed_by: Addr<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>,
}

#[derive(Copy, Clone)]
struct Addr<
    const NUM_ENTRIES: u64,
    const ASSOCIATIVITY: u64,
    const ENTRY_SIZE: u64,
    const ADDR_BITS: u32,
>(u64);

struct Cache<
    const NUM_ENTRIES: u64,
    const ASSOCIATIVITY: u64,
    const ENTRY_SIZE: u64,
    const ADDR_BITS: u32,
> {
    state: Vec<Option<Bucket<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>>>,
}

impl<
        const NUM_ENTRIES: u64,
        const ASSOCIATIVITY: u64,
        const ENTRY_SIZE: u64,
        const ADDR_BITS: u32,
    > Addr<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>
{
    const OFFSET_BITS: u32 = get_log_2(ENTRY_SIZE);
    const INDEX_BITS: u32 = get_log_2(NUM_ENTRIES / ASSOCIATIVITY);
    const TAG_BITS: u32 = ADDR_BITS - Self::OFFSET_BITS - Self::INDEX_BITS;

    const OFFSET_MASK: u64 = (1 << Self::OFFSET_BITS) - 1;

    const INDEX_MASK: u64 = ((1 << (Self::INDEX_BITS + Self::OFFSET_BITS)) - 1) ^ Self::OFFSET_MASK;
    const TAG_MASK: u64 = ((1 << (Self::TAG_BITS + Self::INDEX_BITS + Self::OFFSET_BITS)) - 1)
        ^ Self::INDEX_MASK
        ^ Self::OFFSET_MASK;

    fn offset(&self) -> u64 {
        self.0 & Self::OFFSET_MASK
    }

    fn index(&self) -> u64 {
        (self.0 & Self::INDEX_MASK) >> Self::OFFSET_BITS
    }

    fn tag(&self) -> u64 {
        (self.0 & Self::TAG_MASK) >> (Self::OFFSET_BITS + Self::INDEX_BITS)
    }
}

impl<
        const NUM_ENTRIES: u64,
        const ASSOCIATIVITY: u64,
        const ENTRY_SIZE: u64,
        const ADDR_BITS: u32,
    > Cache<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>
{
    const OFFSET_BITS: u32 = get_log_2(ENTRY_SIZE);
    const INDEX_BITS: u32 = get_log_2(NUM_ENTRIES / ASSOCIATIVITY);
    const TAG_BITS: u32 = ADDR_BITS - Self::OFFSET_BITS - Self::INDEX_BITS;

    fn new() -> Self {
        Cache {
            state: vec![None; usize::try_from(NUM_ENTRIES).unwrap()],
        }
    }

    fn access(&mut self, addr: Addr<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>) {
        assert!(get_largest_bit_pos(addr.0) <= ADDR_BITS);

        let offset = addr.offset();
        let index = addr.index();
        let tag = addr.tag();

        assert!(get_largest_bit_pos(offset) <= Self::OFFSET_BITS);
        assert!(get_largest_bit_pos(index) <= Self::INDEX_BITS);
        assert!(get_largest_bit_pos(tag) <= Self::TAG_BITS);

        print!("addr = ");
        let string = format!("{:010b}", addr.0);
        for (pos, c) in string.chars().enumerate() {
            let i: u32 = (10 - pos - 1).try_into().unwrap();
            if (i == Self::OFFSET_BITS) || (i == Self::OFFSET_BITS + Self::INDEX_BITS) {
                print!("{}|", c);
            } else {
                print!("{}", c);
            }
        }
        println!();
        println!("tag = {}, index = {}, offset = {}", tag, index, offset);

        let index: usize = index.try_into().unwrap();
        if let Some(entry_tag) = self.state[index] {
            if entry_tag.tag == tag {
                println!("hit");
            } else {
                println!("miss, wrong tag");
                self.state[index] = Some(Bucket {
                    tag,
                    last_accessed_by: addr,
                });
            }
        } else {
            println!("miss, invaid");
            self.state[index] = Some(Bucket {
                tag,
                last_accessed_by: addr,
            });
        }
    }
}

impl<
        const NUM_ENTRIES: u64,
        const ASSOCIATIVITY: u64,
        const ENTRY_SIZE: u64,
        const ADDR_BITS: u32,
    > Addr<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>
{
}

const fn get_log_2(x: u64) -> u32 {
    assert!(x.count_ones() == 1);
    x.ilog2()
}

fn get_largest_bit_pos(x: u64) -> u32 {
    u64::BITS - x.leading_zeros()
}
