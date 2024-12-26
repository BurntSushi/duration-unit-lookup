use rand::{RngCore, thread_rng};

const HASH_BITS: u32 = 7;

fn bytes_to_u64(bytes: &[u8]) -> u64 {
    let len = bytes.len().min(8);
    let mut buf = [0u8; 8];
    buf[..len].copy_from_slice(&bytes[..len]);
    u64::from_ne_bytes(buf)
}

fn bad_hash(s: &str, seed: u64) -> usize {
    let x = bytes_to_u64(s.as_bytes()).wrapping_add(s.len() as u64);
    (x.wrapping_mul(seed) >> (64 - HASH_BITS)) as usize
}

fn try_seed(seed: u64) -> bool {
    let mut buckets = [false; 1 << HASH_BITS];
    for (s, _u) in LABELS {
        let i = bad_hash(s, seed);
        if buckets[i] {
            return false;
        }
        buckets[i] = true;
    }
    true
}

fn output_table(seed: u64) {
    let mut strings = [[0u64; 2]; 1 << HASH_BITS];
    let mut units = [Unit::Nanosecond; 1 << HASH_BITS];
    for (s, u) in LABELS {
        let bytes = s.as_bytes();
        let i = bad_hash(s, seed);
        let s0 = bytes_to_u64(bytes);
        let s1 = bytes_to_u64(&bytes[s.len().min(8)..s.len().min(16)]);
        strings[i] = [s0, s1];
        units[i] = *u;
    }
    println!("static STRINGS: [[u64; 2]; {}] = [", 1 << HASH_BITS);
    for i in 0..1 << HASH_BITS {
        println!("    [0x{:x}, 0x{:x}],", strings[i][0], strings[i][1]);
    }
    println!("];");
    println!("static UNITS: [Unit; {}] = [", 1 << HASH_BITS);
    for i in 0..1 << HASH_BITS {
        println!("    Unit::{:?},", units[i]);
    }
    println!("];");
}

fn main() {
    let mut rng = thread_rng();
    for i in 0u64..(1 << 32) {
        let seed = rng.next_u64();
        if try_seed(seed) {
            println!("Seed 0x{seed:x} success after {i} tries");
            output_table(seed);
            break;
        }
    }
}

/// The same `Unit` type used in Jiff, reproduced here for readability
/// mostly.
#[derive(Clone, Copy, Debug)]
enum Unit {
    Year = 9,
    Month = 8,
    Week = 7,
    Day = 6,
    Hour = 5,
    Minute = 4,
    Second = 3,
    Millisecond = 2,
    Microsecond = 1,
    Nanosecond = 0,
}

static LABELS: &[(&str, Unit)] = &[
    ("years", Unit::Year),
    ("year", Unit::Year),
    ("yrs", Unit::Year),
    ("yr", Unit::Year),
    ("y", Unit::Year),
    ("months", Unit::Month),
    ("month", Unit::Month),
    ("mos", Unit::Month),
    ("mo", Unit::Month),
    ("weeks", Unit::Week),
    ("week", Unit::Week),
    ("wks", Unit::Week),
    ("wk", Unit::Week),
    ("w", Unit::Week),
    ("days", Unit::Day),
    ("day", Unit::Day),
    ("d", Unit::Day),
    ("hours", Unit::Hour),
    ("hour", Unit::Hour),
    ("hrs", Unit::Hour),
    ("hr", Unit::Hour),
    ("h", Unit::Hour),
    ("minutes", Unit::Minute),
    ("minute", Unit::Minute),
    ("mins", Unit::Minute),
    ("min", Unit::Minute),
    ("m", Unit::Minute),
    ("seconds", Unit::Second),
    ("second", Unit::Second),
    ("secs", Unit::Second),
    ("sec", Unit::Second),
    ("s", Unit::Second),
    ("milliseconds", Unit::Millisecond),
    ("millisecond", Unit::Millisecond),
    ("millis", Unit::Millisecond),
    ("milli", Unit::Millisecond),
    ("msecs", Unit::Millisecond),
    ("msec", Unit::Millisecond),
    ("ms", Unit::Millisecond),
    ("microseconds", Unit::Microsecond),
    ("microsecond", Unit::Microsecond),
    ("micros", Unit::Microsecond),
    ("micro", Unit::Microsecond),
    ("usecs", Unit::Microsecond),
    ("usec", Unit::Microsecond),
    ("µsecs", Unit::Microsecond),
    ("µsec", Unit::Microsecond),
    ("us", Unit::Microsecond),
    ("µs", Unit::Microsecond),
    ("nanoseconds", Unit::Nanosecond),
    ("nanosecond", Unit::Nanosecond),
    ("nanos", Unit::Nanosecond),
    ("nano", Unit::Nanosecond),
    ("nsecs", Unit::Nanosecond),
    ("nsec", Unit::Nanosecond),
    ("ns", Unit::Nanosecond),
];
