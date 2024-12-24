use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    io::Write,
    ops::RangeInclusive,
};

use regex_automata::{
    dfa::{dense::DFA, Automaton, StartKind},
    util::{primitives::StateID, start},
    Anchored, MatchKind,
};

fn main() -> anyhow::Result<()> {
    let mut out = std::io::stdout().lock();
    let labels: Vec<&str> =
        LABELS.iter().map(|(label, _)| label).copied().collect();
    let config = DFA::config()
        .accelerate(false)
        .starts_for_each_pattern(false) // default
        .specialize_start_states(false) // default
        .start_kind(StartKind::Anchored)
        .match_kind(MatchKind::All)
        // This makes it so our alphabet is always the special EOI symbol and
        // every possible byte value. i.e., No alphabet compression.
        .byte_classes(false)
        .minimize(true);
    let dfa = DFA::builder().configure(config).build_many(&labels)?;

    let language = match std::env::args_os().nth(1) {
        None => "--rust".to_string(),
        Some(arg) => arg.to_string_lossy().into_owned(),
    };
    match &*language {
        "--rust" => print_rust_dfa(&dfa, LABELS, &mut out)?,
        "--c" => print_c_dfa(&dfa, LABELS, &mut out)?,
        unk => anyhow::bail!("unknown language choice: {unk}"),
    }
    Ok(())
}

fn print_rust_dfa(
    dfa: &DFA<Vec<u32>>,
    labels: &[(&str, Unit)],
    out: &mut dyn Write,
) -> anyhow::Result<()> {
    let mut trav = TraversalState::default();

    let start =
        dfa.start_state(&start::Config::new().anchored(Anchored::Yes))?;
    trav.enqueue(start);

    writeln!(out, "use crate::Unit;")?;
    writeln!(out, "")?;
    writeln!(out, "#[inline(always)]")?;
    writeln!(
        out,
        "pub(super) fn find(haystack: &[u8]) -> Option<(Unit, usize)> {{"
    )?;
    writeln!(out, "  let mut sid = State::S{};", trav.gen_id(start))?;
    writeln!(out, "  for byte in haystack.iter().copied() {{")?;

    writeln!(out, "    sid = match sid {{")?;
    writeln!(out, "      State::DEAD => return None,")?;
    while let Some(sid) = trav.dequeue() {
        writeln!(out, "      State::S{} => {{", trav.gen_id(sid))?;

        let (trans, all_bytes_covered) =
            StateTransitionIter::new(dfa, sid).into_map();
        if trans.is_empty() {
            // For this particular DFA, we never visit dead/quit states. So
            // any state with no outgoing transitions has to, I believe, be
            // a sentinel for a match state.
            assert!(
                dfa.match_len(sid) > 0,
                "expected {sid:?} with no transitions to be a match state"
            );
            // We ignore any matches past the first. If there were multiple,
            // that would imply two identical designator labels (since we just
            // have plain non-regex needles), which doesn't really make sense
            // for this use case.
            let (label, unit) = labels[dfa.match_pattern(sid, 0)];
            let len = label.len();
            writeln!(out, "        return Some(({unit}, {len}));")?;
        } else {
            writeln!(out, "        match byte {{")?;
            for (next, ranges) in trans {
                trav.enqueue(next);
                write!(out, "          ")?;
                for (i, range) in ranges.into_iter().enumerate() {
                    if i > 0 {
                        write!(out, " | ")?;
                    }
                    write!(out, "{range}")?;
                }
                writeln!(out, " => State::S{},", trav.gen_id(next))?;
            }
            if !all_bytes_covered {
                writeln!(out, "          _ => State::DEAD,")?;
            }
            writeln!(out, "        }}")?;
        }
        writeln!(out, "      }}")?;

        // We didn't handle the EOI transition above intentionally. Instead,
        // we collect all non-dead transitions here. We write out one last
        // `match` statement after the search loop for these.
        let eoi_next = dfa.next_eoi_state(sid);
        if !dfa.is_dead_state(eoi_next) {
            trav.enqueue(eoi_next);
            trav.eois.push(sid);
        }
    }
    writeln!(out, "    }};")?;
    writeln!(out, "  }}")?;

    if trav.eois.is_empty() {
        writeln!(out, "  return None;")?;
    } else {
        writeln!(out, "  return match sid {{")?;
        for sid in trav.eois.clone().into_iter() {
            let eoi_sid = dfa.next_eoi_state(sid);
            assert!(
                dfa.is_match_state(eoi_sid),
                "expected all non-dead EOI transitions to map to a match",
            );

            let gen_id = trav.gen_id(sid);
            let (label, unit) = labels[dfa.match_pattern(eoi_sid, 0)];
            let len = label.len();
            writeln!(out, "    State::S{gen_id} => Some(({unit}, {len})),")?;
        }
        writeln!(out, "    _ => None,")?;
        writeln!(out, "  }};")?;
    }

    writeln!(out, "")?;
    writeln!(out, "  enum State {{")?;
    writeln!(out, "    DEAD,")?;
    for i in 0..trav.state_ids.len() {
        writeln!(out, "    S{i},")?;
    }
    writeln!(out, "  }}")?;

    writeln!(out, "}}")?;

    Ok(())
}

fn print_c_dfa(
    dfa: &DFA<Vec<u32>>,
    labels: &[(&str, Unit)],
    out: &mut dyn Write,
) -> anyhow::Result<()> {
    let mut trav = TraversalState::default();

    let start =
        dfa.start_state(&start::Config::new().anchored(Anchored::Yes))?;
    trav.enqueue(start);

    writeln!(out, "#include <stddef.h>")?;
    writeln!(out, "#include <stdint.h>")?;
    writeln!(out, "")?;
    writeln!(out, "enum unit {{")?;
    writeln!(out, "  Year = 9,")?;
    writeln!(out, "  Month = 8,")?;
    writeln!(out, "  Week = 7,")?;
    writeln!(out, "  Day = 6,")?;
    writeln!(out, "  Hour = 5,")?;
    writeln!(out, "  Minute = 4,")?;
    writeln!(out, "  Second = 3,")?;
    writeln!(out, "  Millisecond = 2,")?;
    writeln!(out, "  Microsecond = 1,")?;
    writeln!(out, "  Nanosecond = 0,")?;
    writeln!(out, "}};")?;
    writeln!(out, "")?;
    writeln!(out, "struct output {{")?;
    writeln!(out, "  enum unit unit;")?;
    writeln!(out, "  size_t length;")?;
    writeln!(out, "}};")?;
    writeln!(out, "")?;
    writeln!(out, "struct output gencdfa1_find(uint8_t *p, uint8_t *end)")?;
    writeln!(out, "{{")?;
    writeln!(out, "  struct output o = {{ .unit = Year, .length = 0 }};")?;

    while let Some(sid) = trav.dequeue() {
        let (trans, all_bytes_covered) =
            StateTransitionIter::new(dfa, sid).into_map();
        let eoi_next = dfa.next_eoi_state(sid);

        // For this specific problem, our DFA is always a directed acyclic word
        // graph (DAWG), so we'll never come back to the start state. Thus, the
        // start state needs no label. (It's harmless, but if we write it, we
        // get a warning about an unused label. So let's be tidy.)
        if sid != start {
            writeln!(out, "S{}:", trav.gen_id(sid))?;
        }

        // As a special case, if all bytes have outgoing transitions and all
        // of those transitions point to the same state as an outgoing EOI
        // transition, then we can skip the bound check and byte matching and
        // just jump straight to the end.
        if all_bytes_covered && trans.iter().all(|(&next, _)| next == eoi_next)
        {
            if dfa.is_match_state(eoi_next) {
                let (label, unit) = labels[dfa.match_pattern(eoi_next, 0)];
                let len = label.len();
                writeln!(out, "  o.unit = {unit:?};")?;
                writeln!(out, "  o.length = {len};")?;
            }
            writeln!(out, "  goto DONE;")?;
            continue;
        }

        if trans.is_empty() {
            // For this particular DFA, we never visit dead/quit states. So
            // any state with no outgoing transitions has to, I believe, be
            // a sentinel for a match state.
            assert!(
                dfa.match_len(sid) > 0,
                "expected {sid:?} with no transitions to be a match state"
            );
            // We ignore any matches past the first. If there were multiple,
            // that would imply two identical designator labels (since we just
            // have plain non-regex needles), which doesn't really make sense
            // for this use case.
            let (label, unit) = labels[dfa.match_pattern(sid, 0)];
            let len = label.len();
            writeln!(out, "  o.unit = {unit:?};")?;
            writeln!(out, "  o.length = {len};")?;
            writeln!(out, "  goto DONE;")?;
        } else {
            writeln!(out, "  if (p >= end) {{")?;
            if dfa.is_dead_state(eoi_next) {
                writeln!(out, "    goto DONE;")?;
            } else {
                trav.enqueue(eoi_next);
                writeln!(out, "    goto S{};", trav.gen_id(eoi_next))?;
            }
            writeln!(out, "  }}")?;

            writeln!(out, "  switch (*p++) {{")?;
            for (next, ranges) in trans {
                trav.enqueue(next);
                for range in ranges {
                    writeln!(
                        out,
                        "    case {range}: goto S{next};",
                        range = range.to_c(),
                        next = trav.gen_id(next),
                    )?;
                }
            }
            if !all_bytes_covered {
                writeln!(out, "    default: goto DONE;")?;
            }
            writeln!(out, "  }}")?;
        }
    }

    writeln!(out, "DONE:")?;
    writeln!(out, "  return o;")?;
    writeln!(out, "}}")?;

    Ok(())
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

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unit::{self:?}")
    }
}

/// Some state grouped together for facilitating the traversal of a DFA.
#[derive(Debug, Default)]
struct TraversalState {
    /// A map from regex-automata's DFA state ID to the state IDs used in the
    /// generated code. The generated code state IDs are contiguous starting
    /// from zero and have no special significance. The regex-automata DFA
    /// state IDs are not contiguous (because they are pre-multiplified).
    state_ids: HashMap<StateID, usize>,
    /// States that have an outgoing EOI transition.
    ///
    /// For our particular use case, these EOI transitions always map to
    /// match states.
    eois: Vec<StateID>,
    /// The queue of states to visit. Pushing to this queue is guarded by
    /// checking `seen` if the state has been visited.
    queue: VecDeque<StateID>,
    /// The set of state IDs we've visited. No need to visit a state more than
    /// once.
    seen: HashSet<StateID>,
}

impl TraversalState {
    /// Push the given state on to the queue of states to visit.
    ///
    /// This is a no-op if the given state ID has been pushed to the queue
    /// before (thus, a state can never be revisited).
    fn enqueue(&mut self, sid: StateID) {
        if self.seen.insert(sid) {
            self.gen_id(sid);
            self.queue.push_back(sid);
        }
    }

    /// Pop a state ID from the queue.
    fn dequeue(&mut self) -> Option<StateID> {
        self.queue.pop_front()
    }

    /// Return the ID of the given state for use in the generated code.
    ///
    /// If one doesn't exist for the state given, then one is generated.
    ///
    /// This maintains a contiguous sequence of state IDs for use in the
    /// generated code.
    fn gen_id(&mut self, sid: StateID) -> usize {
        if let Some(&id) = self.state_ids.get(&sid) {
            return id;
        }
        let id = self.state_ids.len();
        self.state_ids.insert(sid, id);
        id
    }
}

/// Copied from `regex-automata`. This is used to print the `Debug`
/// representation of a DFA in `regex-automata`. We adapt it slightly to
/// iterate over distinct byte values instead of the actual transitions in the
/// state (which should be equivalent).
///
/// This does not include the EOI transition, which we handle specially.
#[derive(Debug)]
struct StateTransitionIter<'a> {
    dfa: &'a DFA<Vec<u32>>,
    sid: StateID,
    alphabet: RangeInclusive<u8>,
    cur: Option<ContiguousTransitions>,
}

impl<'a> StateTransitionIter<'a> {
    fn new(dfa: &'a DFA<Vec<u32>>, sid: StateID) -> StateTransitionIter<'a> {
        StateTransitionIter { dfa, sid, alphabet: 0..=255, cur: None }
    }

    fn into_map(self) -> (BTreeMap<StateID, Vec<ByteRange>>, bool) {
        let mut map: BTreeMap<StateID, Vec<ByteRange>> = BTreeMap::new();
        let mut covered_bytes = [false; 256];
        for ContiguousTransitions { start, end, next } in self {
            for byte in start..=end {
                covered_bytes[usize::from(byte)] = true;
            }
            map.entry(next).or_default().push(ByteRange(start, end));
        }
        (map, covered_bytes.into_iter().all(|covered| covered))
    }
}

impl<'a> Iterator for StateTransitionIter<'a> {
    type Item = ContiguousTransitions;

    fn next(&mut self) -> Option<ContiguousTransitions> {
        while let Some(byte) = self.alphabet.next() {
            let next = self.dfa.next_state(self.sid, byte);
            let prev = match self.cur {
                Some(prev) => prev,
                None => {
                    self.cur = Some(ContiguousTransitions {
                        start: byte,
                        end: byte,
                        next,
                    });
                    continue;
                }
            };
            if prev.next == next {
                self.cur = Some(ContiguousTransitions { end: byte, ..prev });
            } else {
                self.cur = Some(ContiguousTransitions {
                    start: byte,
                    end: byte,
                    next,
                });
                if !self.dfa.is_dead_state(prev.next) {
                    return Some(prev);
                }
            }
        }
        if let Some(trans) = self.cur.take() {
            if !self.dfa.is_dead_state(trans.next) {
                return Some(trans);
            }
        }
        None
    }
}

/// A helper type for formatting a single contiguous range of bytes as a
/// range in a Rust `match` expression.
struct ByteRange(u8, u8);

impl ByteRange {
    fn to_c(self) -> CByteRange {
        CByteRange(self.0, self.1)
    }
}

impl std::fmt::Display for ByteRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 == self.1 {
            std::fmt::Display::fmt(&ByteLiteral(self.0), f)
        } else {
            write!(f, "{}..={}", ByteLiteral(self.0), ByteLiteral(self.1))
        }
    }
}

/// A helper type for formatting a single contiguous range of bytes as a
/// range in a C `case` block (part of a `switch` statement).
struct CByteRange(u8, u8);

impl std::fmt::Display for CByteRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 == self.1 {
            std::fmt::Display::fmt(&CByteLiteral(self.0), f)
        } else {
            write!(f, "{} ... {}", CByteLiteral(self.0), CByteLiteral(self.1))
        }
    }
}

/// A helper type for formatting a byte literal in Rust source.
///
/// We could just always use hex escapes, but I like using the actual
/// ASCII characters for printable characters since it makes reading the
/// generated code easier.
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

/// A helper type for formatting a byte literal in C source.
struct CByteLiteral(u8);

impl std::fmt::Display for CByteLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if b'a' <= self.0 && self.0 <= b'z' {
            write!(f, "'{}'", self.0 as char)
        } else {
            write!(f, "0x{:02x}", self.0)
        }
    }
}

/// A helper type for combining a range of bytes (inclusive) with its
/// corresponding outgoing transition state ID.
#[derive(Clone, Copy, Debug)]
struct ContiguousTransitions {
    start: u8,
    end: u8,
    next: StateID,
}

// static LABELS: &[(&str, Unit)] = &[("months", Unit::Month)];

// static LABELS: &[(&str, Unit)] = &[
// ("years", Unit::Year),
// ("year", Unit::Year),
// ("yrs", Unit::Year),
// ("yr", Unit::Year),
// ("y", Unit::Year),
// ];

// static LABELS: &[(&str, Unit)] =
// &[("yrs", Unit::Year), ("mos", Unit::Month), ("hrs", Unit::Hour)];

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
