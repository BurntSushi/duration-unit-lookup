use crate::{
    trie3::{Trie, TrieNeedles},
    Unit,
};

type DesignatorTrie = Trie<99, { DESIGNATOR_NEEDLES.alphabet_len() }, Unit>;

static DESIGNATOR_TRIE: &'static DesignatorTrie =
    &Trie::new(&DESIGNATOR_NEEDLES);

const DESIGNATOR_NEEDLES: TrieNeedles<Unit> = TrieNeedles::new(&[
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
]);

#[inline(never)]
pub fn lookup<'i>(input: &'i [u8]) -> Result<(Unit, &'i [u8]), anyhow::Error> {
    let Some((unit, offset)) = DESIGNATOR_TRIE.find(input) else {
        if input.is_empty() {
            anyhow::bail!(
                "expected to find unit designator suffix \
                 (e.g., 'years' or 'secs'), \
                 but found end of input",
            );
        } else {
            anyhow::bail!(
                "expected to find unit designator suffix \
                 (e.g., 'years' or 'secs'), \
                 but found input beginning with {found:?} instead",
                found = std::str::from_utf8(input).unwrap_or("N/A"),
            );
        }
    };
    Ok((unit, &input[offset..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        dbg!(std::mem::size_of::<DesignatorTrie>());
    }
}
