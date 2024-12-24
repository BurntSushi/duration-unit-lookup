use std::io::Write;

fn main() -> anyhow::Result<()> {
    let mut labels = LABELS.iter().copied().collect::<Vec<(&str, Unit)>>();
    labels.sort_by(|&(lab1, _), &(lab2, _)| {
        (lab1.len(), lab1).cmp(&(lab2.len(), lab2)).reverse()
    });

    let mut out = std::io::stdout().lock();
    writeln!(out, "use crate::Unit;")?;
    writeln!(out, "")?;
    writeln!(out, "#[inline(always)]")?;
    writeln!(
        out,
        "pub(super) fn find(haystack: &[u8]) -> Option<(Unit, usize)> {{"
    )?;
    writeln!(out, "  match haystack {{")?;
    for (label, unit) in labels {
        write!(out, "    &[")?;
        for &byte in label.as_bytes() {
            write!(out, "{}, ", ByteLiteral(byte))?;
        }
        writeln!(out, "..] => Some((Unit::{unit:?}, {})),", label.len())?;
    }
    writeln!(out, "  _ => None,")?;
    writeln!(out, "  }}")?;
    writeln!(out, "}}")?;
    Ok(())
}

/// A helper type for formatting a byte literal in Rust source.
struct ByteLiteral(u8);

impl std::fmt::Display for ByteLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "b'")?;
        for escaped_byte in std::ascii::escape_default(self.0) {
            write!(f, "{}", escaped_byte as char)?;
        }
        write!(f, "'")?;
        Ok(())
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
