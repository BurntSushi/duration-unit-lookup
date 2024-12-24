use crate::Unit;

#[inline(always)]
pub(super) fn find(haystack: &[u8]) -> Option<(Unit, usize)> {
    match haystack {
        &[b'm', b'i', b'l', b'l', b'i', b's', b'e', b'c', b'o', b'n', b'd', b's', ..] => {
            Some((Unit::Millisecond, 12))
        }
        &[b'm', b'i', b'c', b'r', b'o', b's', b'e', b'c', b'o', b'n', b'd', b's', ..] => {
            Some((Unit::Microsecond, 12))
        }
        &[b'n', b'a', b'n', b'o', b's', b'e', b'c', b'o', b'n', b'd', b's', ..] => {
            Some((Unit::Nanosecond, 11))
        }
        &[b'm', b'i', b'l', b'l', b'i', b's', b'e', b'c', b'o', b'n', b'd', ..] => {
            Some((Unit::Millisecond, 11))
        }
        &[b'm', b'i', b'c', b'r', b'o', b's', b'e', b'c', b'o', b'n', b'd', ..] => {
            Some((Unit::Microsecond, 11))
        }
        &[b'n', b'a', b'n', b'o', b's', b'e', b'c', b'o', b'n', b'd', ..] => {
            Some((Unit::Nanosecond, 10))
        }
        &[b's', b'e', b'c', b'o', b'n', b'd', b's', ..] => {
            Some((Unit::Second, 7))
        }
        &[b'm', b'i', b'n', b'u', b't', b'e', b's', ..] => {
            Some((Unit::Minute, 7))
        }
        &[b'\xc2', b'\xb5', b's', b'e', b'c', b's', ..] => {
            Some((Unit::Microsecond, 6))
        }
        &[b's', b'e', b'c', b'o', b'n', b'd', ..] => Some((Unit::Second, 6)),
        &[b'm', b'o', b'n', b't', b'h', b's', ..] => Some((Unit::Month, 6)),
        &[b'm', b'i', b'n', b'u', b't', b'e', ..] => Some((Unit::Minute, 6)),
        &[b'm', b'i', b'l', b'l', b'i', b's', ..] => {
            Some((Unit::Millisecond, 6))
        }
        &[b'm', b'i', b'c', b'r', b'o', b's', ..] => {
            Some((Unit::Microsecond, 6))
        }
        &[b'\xc2', b'\xb5', b's', b'e', b'c', ..] => {
            Some((Unit::Microsecond, 5))
        }
        &[b'y', b'e', b'a', b'r', b's', ..] => Some((Unit::Year, 5)),
        &[b'w', b'e', b'e', b'k', b's', ..] => Some((Unit::Week, 5)),
        &[b'u', b's', b'e', b'c', b's', ..] => Some((Unit::Microsecond, 5)),
        &[b'n', b's', b'e', b'c', b's', ..] => Some((Unit::Nanosecond, 5)),
        &[b'n', b'a', b'n', b'o', b's', ..] => Some((Unit::Nanosecond, 5)),
        &[b'm', b's', b'e', b'c', b's', ..] => Some((Unit::Millisecond, 5)),
        &[b'm', b'o', b'n', b't', b'h', ..] => Some((Unit::Month, 5)),
        &[b'm', b'i', b'l', b'l', b'i', ..] => Some((Unit::Millisecond, 5)),
        &[b'm', b'i', b'c', b'r', b'o', ..] => Some((Unit::Microsecond, 5)),
        &[b'h', b'o', b'u', b'r', b's', ..] => Some((Unit::Hour, 5)),
        &[b'y', b'e', b'a', b'r', ..] => Some((Unit::Year, 4)),
        &[b'w', b'e', b'e', b'k', ..] => Some((Unit::Week, 4)),
        &[b'u', b's', b'e', b'c', ..] => Some((Unit::Microsecond, 4)),
        &[b's', b'e', b'c', b's', ..] => Some((Unit::Second, 4)),
        &[b'n', b's', b'e', b'c', ..] => Some((Unit::Nanosecond, 4)),
        &[b'n', b'a', b'n', b'o', ..] => Some((Unit::Nanosecond, 4)),
        &[b'm', b's', b'e', b'c', ..] => Some((Unit::Millisecond, 4)),
        &[b'm', b'i', b'n', b's', ..] => Some((Unit::Minute, 4)),
        &[b'h', b'o', b'u', b'r', ..] => Some((Unit::Hour, 4)),
        &[b'd', b'a', b'y', b's', ..] => Some((Unit::Day, 4)),
        &[b'\xc2', b'\xb5', b's', ..] => Some((Unit::Microsecond, 3)),
        &[b'y', b'r', b's', ..] => Some((Unit::Year, 3)),
        &[b'w', b'k', b's', ..] => Some((Unit::Week, 3)),
        &[b's', b'e', b'c', ..] => Some((Unit::Second, 3)),
        &[b'm', b'o', b's', ..] => Some((Unit::Month, 3)),
        &[b'm', b'i', b'n', ..] => Some((Unit::Minute, 3)),
        &[b'h', b'r', b's', ..] => Some((Unit::Hour, 3)),
        &[b'd', b'a', b'y', ..] => Some((Unit::Day, 3)),
        &[b'y', b'r', ..] => Some((Unit::Year, 2)),
        &[b'w', b'k', ..] => Some((Unit::Week, 2)),
        &[b'u', b's', ..] => Some((Unit::Microsecond, 2)),
        &[b'n', b's', ..] => Some((Unit::Nanosecond, 2)),
        &[b'm', b's', ..] => Some((Unit::Millisecond, 2)),
        &[b'm', b'o', ..] => Some((Unit::Month, 2)),
        &[b'h', b'r', ..] => Some((Unit::Hour, 2)),
        &[b'y', ..] => Some((Unit::Year, 1)),
        &[b'w', ..] => Some((Unit::Week, 1)),
        &[b's', ..] => Some((Unit::Second, 1)),
        &[b'm', ..] => Some((Unit::Minute, 1)),
        &[b'h', ..] => Some((Unit::Hour, 1)),
        &[b'd', ..] => Some((Unit::Day, 1)),
        _ => None,
    }
}
