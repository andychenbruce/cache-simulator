fn main() {
    let mut cache: Cache<4, 2, 8, 8> = Cache::new();

    for addr in [29, 26, 45, 61, 29, 58, 232, 125, 29, 61] {
        cache.access(Addr(addr));
    }

    println!("FINAL STATE:");
    for (bucket_num, bucket) in cache.state.iter().enumerate() {
        println!("bucket {} ======================", bucket_num);
        for entry in bucket.entries.iter() {
            if let Some(entry) = entry {
                println!("entry: {}", entry.last_accessed_by.0);
            } else {
                println!("entry: INVALID");
            }
        }
        println!();
    }
}

#[derive(Copy, Clone)]
struct Bucket<
    const NUM_ENTRIES: u64,
    const ASSOCIATIVITY: usize,
    const ENTRY_SIZE: u64,
    const ADDR_BITS: u32,
> {
    entries: [Option<Entry<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>>; ASSOCIATIVITY],
}

#[derive(Copy, Clone)]
struct Entry<
    const NUM_ENTRIES: u64,
    const ASSOCIATIVITY: usize,
    const ENTRY_SIZE: u64,
    const ADDR_BITS: u32,
> {
    time_since_last_hit: u32,
    tag: u64,
    last_accessed_by: Addr<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>,
}

impl<
        const NUM_ENTRIES: u64,
        const ASSOCIATIVITY: usize,
        const ENTRY_SIZE: u64,
        const ADDR_BITS: u32,
    > Bucket<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>
{
    fn new() -> Self {
        Self {
            entries: [None; ASSOCIATIVITY],
        }
    }
    fn access(
        &mut self,
        addr: Addr<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>,
    ) -> AccessResult {
        let tag = addr.tag();

        for entry in self.entries.iter_mut().flatten() {
            entry.time_since_last_hit += 1;
        }

        if let Some(entry) = self
            .entries
            .iter_mut()
            .filter_map(|x| x.as_mut())
            .find(|entry| entry.tag == tag)
        {
            entry.time_since_last_hit = 0;
            return AccessResult::Hit;
        }
        if let Some(first_empty) = self.entries.iter_mut().find(|x| x.is_none()) {
            *first_empty = Some(Entry {
                time_since_last_hit: 0,
                tag,
                last_accessed_by: addr,
            });

            AccessResult::MissInvalid
        } else {
            let lru = self
                .entries
                .iter_mut()
                .filter_map(|x| x.as_mut())
                .max_by_key(|x| x.time_since_last_hit)
                .unwrap();

            *lru = Entry {
                time_since_last_hit: 0,
                tag,
                last_accessed_by: addr,
            };

            AccessResult::MissWrongTag
        }
    }
}

enum AccessResult {
    Hit,
    MissWrongTag,
    MissInvalid,
}

#[derive(Copy, Clone)]
struct Addr<
    const NUM_ENTRIES: u64,
    const ASSOCIATIVITY: usize,
    const ENTRY_SIZE: u64,
    const ADDR_BITS: u32,
>(u64);

struct Cache<
    const NUM_ENTRIES: u64,
    const ASSOCIATIVITY: usize,
    const ENTRY_SIZE: u64,
    const ADDR_BITS: u32,
> {
    state: Vec<Bucket<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>>,
}

impl<
        const NUM_ENTRIES: u64,
        const ASSOCIATIVITY: usize,
        const ENTRY_SIZE: u64,
        const ADDR_BITS: u32,
    > Addr<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>
{
    const OFFSET_BITS: u32 = get_log_2(ENTRY_SIZE);
    const INDEX_BITS: u32 = get_log_2(NUM_ENTRIES / (ASSOCIATIVITY as u64));
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

    fn print_str(&self) -> String {
        let mut output = "".to_string();

        let bits = (0..ADDR_BITS).map(|bit_pos| (self.0 & (1 << bit_pos) != 0));

        for (pos, bit) in bits.enumerate().rev() {
            let c = if bit { '1' } else { '0' };
            let pos: u32 = pos.try_into().unwrap();
            if (pos == Self::OFFSET_BITS) || (pos == Self::OFFSET_BITS + Self::INDEX_BITS) {
                output += &format!("{}|", c);
            } else {
                output += &format!("{}", c);
            }
        }

        output
    }
}

impl<
        const NUM_ENTRIES: u64,
        const ASSOCIATIVITY: usize,
        const ENTRY_SIZE: u64,
        const ADDR_BITS: u32,
    > Cache<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>
{
    fn new() -> Self {
        Cache {
            state: vec![Bucket::new(); usize::try_from(NUM_ENTRIES).unwrap() / ASSOCIATIVITY],
        }
    }

    fn access(&mut self, addr: Addr<NUM_ENTRIES, ASSOCIATIVITY, ENTRY_SIZE, ADDR_BITS>) {
        assert!(get_largest_bit_pos(addr.0) <= ADDR_BITS);

        let offset = addr.offset();
        let index = addr.index();
        let tag = addr.tag();

        println!("addr = {}", addr.print_str());
        println!("tag = {}, index = {}, offset = {}", tag, index, offset);

        let index: usize = index.try_into().unwrap();
        let access_state = self.state[index].access(addr);
        match access_state {
            AccessResult::Hit => println!("HIT"),
            AccessResult::MissWrongTag => println!("MISS: wrong tag"),
            AccessResult::MissInvalid => println!("MISS: invalid"),
        }
    }
}

impl<
        const NUM_ENTRIES: u64,
        const ASSOCIATIVITY: usize,
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
