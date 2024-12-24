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
    let unit = match needle {
        b"years" => Unit::Year,
        b"year" => Unit::Year,
        b"yrs" => Unit::Year,
        b"yr" => Unit::Year,
        b"y" => Unit::Year,
        b"months" => Unit::Month,
        b"month" => Unit::Month,
        b"mos" => Unit::Month,
        b"mo" => Unit::Month,
        b"weeks" => Unit::Week,
        b"week" => Unit::Week,
        b"wks" => Unit::Week,
        b"wk" => Unit::Week,
        b"w" => Unit::Week,
        b"days" => Unit::Day,
        b"day" => Unit::Day,
        b"d" => Unit::Day,
        b"hours" => Unit::Hour,
        b"hour" => Unit::Hour,
        b"hrs" => Unit::Hour,
        b"hr" => Unit::Hour,
        b"h" => Unit::Hour,
        b"minutes" => Unit::Minute,
        b"minute" => Unit::Minute,
        b"mins" => Unit::Minute,
        b"min" => Unit::Minute,
        b"m" => Unit::Minute,
        b"seconds" => Unit::Second,
        b"second" => Unit::Second,
        b"secs" => Unit::Second,
        b"sec" => Unit::Second,
        b"s" => Unit::Second,
        b"milliseconds" => Unit::Millisecond,
        b"millisecond" => Unit::Millisecond,
        b"millis" => Unit::Millisecond,
        b"milli" => Unit::Millisecond,
        b"msecs" => Unit::Millisecond,
        b"msec" => Unit::Millisecond,
        b"ms" => Unit::Millisecond,
        b"microseconds" => Unit::Microsecond,
        b"microsecond" => Unit::Microsecond,
        b"micros" => Unit::Microsecond,
        b"micro" => Unit::Microsecond,
        b"usecs" => Unit::Microsecond,
        b"usec" => Unit::Microsecond,
        b"\xC2\xB5secs" => Unit::Microsecond,
        b"\xC2\xB5sec" => Unit::Microsecond,
        b"us" => Unit::Microsecond,
        b"\xC2\xB5s" => Unit::Microsecond,
        b"nanoseconds" => Unit::Nanosecond,
        b"nanosecond" => Unit::Nanosecond,
        b"nanos" => Unit::Nanosecond,
        b"nano" => Unit::Nanosecond,
        b"nsecs" => Unit::Nanosecond,
        b"nsec" => Unit::Nanosecond,
        b"ns" => Unit::Nanosecond,
        unk => {
            anyhow::bail!(
                "expected to find unit designator suffix \
                 (e.g., 'years' or 'secs'), \
                 but found input beginning with {found:?} instead",
                found = std::str::from_utf8(unk).unwrap_or("N/A"),
            );
        }
    };
    Ok((unit, &input[end..]))
}
