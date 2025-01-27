use crate::Unit;

#[inline(never)]
pub fn lookup<'i>(input: &'i [u8]) -> Result<(Unit, &'i [u8]), anyhow::Error> {
    let is_valid_designator_byte =
            // The \xC2 and \xB5 is for `µ`.
            |b: u8| b'a' <= b && b <= b'z' || b == 0xC2 || b == 0xB5;
    let end = input
        .iter()
        .position(|&b| !is_valid_designator_byte(b))
        .unwrap_or(input.len());
    let needle = &input[..end];
    if needle.is_empty() {
        anyhow::bail!(
            "expected to find unit designator suffix \
             (e.g., 'years' or 'secs'), \
             but found end of input",
        );
    }
    let unit = find(needle).ok_or_else(|| {
        anyhow::anyhow!(
            "expected to find unit designator suffix \
             (e.g., 'years' or 'secs'), \
             but found input beginning with {found:?} instead",
            found = std::str::from_utf8(needle).unwrap_or("N/A"),
        )
    })?;
    Ok((unit, &input[end..]))
}

#[inline(always)]
fn find(input: &[u8]) -> Option<Unit> {
    hashify::tiny_map! {
        input,
        "years" => Unit::Year,
        "year" => Unit::Year,
        "yrs" => Unit::Year,
        "yr" => Unit::Year,
        "y" => Unit::Year,
        "months" => Unit::Month,
        "month" => Unit::Month,
        "mos" => Unit::Month,
        "mo" => Unit::Month,
        "weeks" => Unit::Week,
        "week" => Unit::Week,
        "wks" => Unit::Week,
        "wk" => Unit::Week,
        "w" => Unit::Week,
        "days" => Unit::Day,
        "day" => Unit::Day,
        "d" => Unit::Day,
        "hours" => Unit::Hour,
        "hour" => Unit::Hour,
        "hrs" => Unit::Hour,
        "hr" => Unit::Hour,
        "h" => Unit::Hour,
        "minutes" => Unit::Minute,
        "minute" => Unit::Minute,
        "mins" => Unit::Minute,
        "min" => Unit::Minute,
        "m" => Unit::Minute,
        "seconds" => Unit::Second,
        "second" => Unit::Second,
        "secs" => Unit::Second,
        "sec" => Unit::Second,
        "s" => Unit::Second,
        "milliseconds" => Unit::Millisecond,
        "millisecond" => Unit::Millisecond,
        "millis" => Unit::Millisecond,
        "milli" => Unit::Millisecond,
        "msecs" => Unit::Millisecond,
        "msec" => Unit::Millisecond,
        "ms" => Unit::Millisecond,
        "microseconds" => Unit::Microsecond,
        "microsecond" => Unit::Microsecond,
        "micros" => Unit::Microsecond,
        "micro" => Unit::Microsecond,
        "usecs" => Unit::Microsecond,
        "usec" => Unit::Microsecond,
        "µsecs" => Unit::Microsecond,
        "µsec" => Unit::Microsecond,
        "us" => Unit::Microsecond,
        "µs" => Unit::Microsecond,
        "nanoseconds" => Unit::Nanosecond,
        "nanosecond" => Unit::Nanosecond,
        "nanos" => Unit::Nanosecond,
        "nano" => Unit::Nanosecond,
        "nsecs" => Unit::Nanosecond,
        "nsec" => Unit::Nanosecond,
        "ns" => Unit::Nanosecond,
    }
}
