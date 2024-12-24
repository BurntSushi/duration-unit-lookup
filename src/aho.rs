use std::sync::LazyLock;

use aho_corasick::{
    automaton::Automaton, dfa::DFA, Anchored, Input, MatchKind, StartKind,
};

use crate::Unit;

static PATTERNID_TO_UNIT: &'static [Unit] = &[
    Unit::Year,
    Unit::Year,
    Unit::Year,
    Unit::Year,
    Unit::Year,
    Unit::Month,
    Unit::Month,
    Unit::Month,
    Unit::Month,
    Unit::Week,
    Unit::Week,
    Unit::Week,
    Unit::Week,
    Unit::Week,
    Unit::Day,
    Unit::Day,
    Unit::Day,
    Unit::Hour,
    Unit::Hour,
    Unit::Hour,
    Unit::Hour,
    Unit::Hour,
    Unit::Minute,
    Unit::Minute,
    Unit::Minute,
    Unit::Minute,
    Unit::Minute,
    Unit::Second,
    Unit::Second,
    Unit::Second,
    Unit::Second,
    Unit::Second,
    Unit::Millisecond,
    Unit::Millisecond,
    Unit::Millisecond,
    Unit::Millisecond,
    Unit::Millisecond,
    Unit::Millisecond,
    Unit::Millisecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
];

static NEEDLES: &'static [&'static [u8]] = &[
    b"years",
    b"year",
    b"yrs",
    b"yr",
    b"y",
    b"months",
    b"month",
    b"mos",
    b"mo",
    b"weeks",
    b"week",
    b"wks",
    b"wk",
    b"w",
    b"days",
    b"day",
    b"d",
    b"hours",
    b"hour",
    b"hrs",
    b"hr",
    b"h",
    b"minutes",
    b"minute",
    b"mins",
    b"min",
    b"m",
    b"seconds",
    b"second",
    b"secs",
    b"sec",
    b"s",
    b"milliseconds",
    b"millisecond",
    b"millis",
    b"milli",
    b"msecs",
    b"msec",
    b"ms",
    b"microseconds",
    b"microsecond",
    b"micros",
    b"micro",
    b"usecs",
    b"usec",
    b"\xC2\xB5secs",
    b"\xC2\xB5sec",
    b"us",
    b"\xC2\xB5s",
    b"nanoseconds",
    b"nanosecond",
    b"nanos",
    b"nano",
    b"nsecs",
    b"nsec",
    b"ns",
];

static SEARCHER: LazyLock<DFA> = LazyLock::new(|| {
    DFA::builder()
        .match_kind(MatchKind::LeftmostLongest)
        .start_kind(StartKind::Anchored)
        .build(NEEDLES)
        .unwrap()
});

#[inline(never)]
pub fn lookup<'i>(input: &'i [u8]) -> Result<(Unit, &'i [u8]), anyhow::Error> {
    let search = Input::new(input).anchored(Anchored::Yes);
    let Some(mat) = SEARCHER.try_find(&search).unwrap() else {
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
    Ok((PATTERNID_TO_UNIT[mat.pattern()], &input[mat.end()..]))
}
