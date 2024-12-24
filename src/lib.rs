pub mod aho;
pub mod by_gencdfa1;
pub mod by_gendfa1;
pub mod by_trie1;
pub mod by_trie2;
pub mod by_trie3;
pub mod by_trie4;
pub mod by_trie5;
mod gendfa1;
pub mod one_big_match;
pub mod phf;
mod trie1;
mod trie2;
mod trie3;
mod trie4;
mod trie5;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(C)]
pub enum Unit {
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
