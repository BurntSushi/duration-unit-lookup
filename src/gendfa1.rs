use crate::Unit;

#[inline(always)]
pub(super) fn find(haystack: &[u8]) -> Option<(Unit, usize)> {
  let mut sid = State::S0;
  for byte in haystack.iter().copied() {
    sid = match sid {
      State::DEAD => return None,
      State::S0 => {
        match byte {
          b'd' => State::S1,
          b'h' => State::S2,
          b'm' => State::S3,
          b'n' => State::S4,
          b's' => State::S5,
          b'u' => State::S6,
          b'w' => State::S7,
          b'y' => State::S8,
          b'\xc2' => State::S9,
          _ => State::DEAD,
        }
      }
      State::S1 => {
        match byte {
          b'\x00'..=b'`' | b'b'..=b'\xff' => State::S10,
          b'a' => State::S11,
        }
      }
      State::S2 => {
        match byte {
          b'\x00'..=b'n' | b'p'..=b'q' | b's'..=b'\xff' => State::S12,
          b'o' => State::S13,
          b'r' => State::S14,
        }
      }
      State::S3 => {
        match byte {
          b'\x00'..=b'h' | b'j'..=b'n' | b'p'..=b'r' | b't'..=b'\xff' => State::S15,
          b'i' => State::S16,
          b'o' => State::S17,
          b's' => State::S18,
        }
      }
      State::S4 => {
        match byte {
          b'a' => State::S19,
          b's' => State::S20,
          _ => State::DEAD,
        }
      }
      State::S5 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'\xff' => State::S21,
          b'e' => State::S22,
        }
      }
      State::S6 => {
        match byte {
          b's' => State::S23,
          _ => State::DEAD,
        }
      }
      State::S7 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'j' | b'l'..=b'\xff' => State::S24,
          b'e' => State::S25,
          b'k' => State::S26,
        }
      }
      State::S8 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'q' | b's'..=b'\xff' => State::S27,
          b'e' => State::S28,
          b'r' => State::S29,
        }
      }
      State::S9 => {
        match byte {
          b'\xb5' => State::S30,
          _ => State::DEAD,
        }
      }
      State::S10 => {
        return Some((Unit::Day, 1));
      }
      State::S11 => {
        match byte {
          b'y' => State::S31,
          _ => State::DEAD,
        }
      }
      State::S12 => {
        return Some((Unit::Hour, 1));
      }
      State::S13 => {
        match byte {
          b'u' => State::S32,
          _ => State::DEAD,
        }
      }
      State::S14 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S33,
          b's' => State::S34,
        }
      }
      State::S15 => {
        return Some((Unit::Minute, 1));
      }
      State::S16 => {
        match byte {
          b'n' => State::S35,
          b'l' => State::S36,
          b'c' => State::S37,
          _ => State::DEAD,
        }
      }
      State::S17 => {
        match byte {
          b'\x00'..=b'm' | b'o'..=b'r' | b't'..=b'\xff' => State::S38,
          b'n' => State::S39,
          b's' => State::S40,
        }
      }
      State::S18 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'\xff' => State::S41,
          b'e' => State::S42,
        }
      }
      State::S19 => {
        match byte {
          b'n' => State::S43,
          _ => State::DEAD,
        }
      }
      State::S20 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'\xff' => State::S44,
          b'e' => State::S45,
        }
      }
      State::S21 => {
        return Some((Unit::Second, 1));
      }
      State::S22 => {
        match byte {
          b'c' => State::S46,
          _ => State::DEAD,
        }
      }
      State::S23 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'\xff' => State::S47,
          b'e' => State::S48,
        }
      }
      State::S24 => {
        return Some((Unit::Week, 1));
      }
      State::S25 => {
        match byte {
          b'e' => State::S49,
          _ => State::DEAD,
        }
      }
      State::S26 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S50,
          b's' => State::S51,
        }
      }
      State::S27 => {
        return Some((Unit::Year, 1));
      }
      State::S28 => {
        match byte {
          b'a' => State::S52,
          _ => State::DEAD,
        }
      }
      State::S29 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S53,
          b's' => State::S54,
        }
      }
      State::S30 => {
        match byte {
          b's' => State::S55,
          _ => State::DEAD,
        }
      }
      State::S31 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S56,
          b's' => State::S57,
        }
      }
      State::S32 => {
        match byte {
          b'r' => State::S58,
          _ => State::DEAD,
        }
      }
      State::S33 => {
        return Some((Unit::Hour, 2));
      }
      State::S34 => {
        match byte {
          b'\x00'..=b'\xff' => State::S59,
        }
      }
      State::S35 => {
        match byte {
          b'\x00'..=b'r' | b't' | b'v'..=b'\xff' => State::S60,
          b's' => State::S61,
          b'u' => State::S62,
        }
      }
      State::S36 => {
        match byte {
          b'l' => State::S63,
          _ => State::DEAD,
        }
      }
      State::S37 => {
        match byte {
          b'r' => State::S64,
          _ => State::DEAD,
        }
      }
      State::S38 => {
        return Some((Unit::Month, 2));
      }
      State::S39 => {
        match byte {
          b't' => State::S65,
          _ => State::DEAD,
        }
      }
      State::S40 => {
        match byte {
          b'\x00'..=b'\xff' => State::S66,
        }
      }
      State::S41 => {
        return Some((Unit::Millisecond, 2));
      }
      State::S42 => {
        match byte {
          b'c' => State::S67,
          _ => State::DEAD,
        }
      }
      State::S43 => {
        match byte {
          b'o' => State::S68,
          _ => State::DEAD,
        }
      }
      State::S44 => {
        return Some((Unit::Nanosecond, 2));
      }
      State::S45 => {
        match byte {
          b'c' => State::S69,
          _ => State::DEAD,
        }
      }
      State::S46 => {
        match byte {
          b'\x00'..=b'n' | b'p'..=b'r' | b't'..=b'\xff' => State::S70,
          b'o' => State::S71,
          b's' => State::S72,
        }
      }
      State::S47 => {
        return Some((Unit::Microsecond, 2));
      }
      State::S48 => {
        match byte {
          b'c' => State::S73,
          _ => State::DEAD,
        }
      }
      State::S49 => {
        match byte {
          b'k' => State::S74,
          _ => State::DEAD,
        }
      }
      State::S50 => {
        return Some((Unit::Week, 2));
      }
      State::S51 => {
        match byte {
          b'\x00'..=b'\xff' => State::S75,
        }
      }
      State::S52 => {
        match byte {
          b'r' => State::S76,
          _ => State::DEAD,
        }
      }
      State::S53 => {
        return Some((Unit::Year, 2));
      }
      State::S54 => {
        match byte {
          b'\x00'..=b'\xff' => State::S77,
        }
      }
      State::S55 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'\xff' => State::S78,
          b'e' => State::S79,
        }
      }
      State::S56 => {
        return Some((Unit::Day, 3));
      }
      State::S57 => {
        match byte {
          b'\x00'..=b'\xff' => State::S80,
        }
      }
      State::S58 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S81,
          b's' => State::S82,
        }
      }
      State::S59 => {
        return Some((Unit::Hour, 3));
      }
      State::S60 => {
        return Some((Unit::Minute, 3));
      }
      State::S61 => {
        match byte {
          b'\x00'..=b'\xff' => State::S83,
        }
      }
      State::S62 => {
        match byte {
          b't' => State::S84,
          _ => State::DEAD,
        }
      }
      State::S63 => {
        match byte {
          b'i' => State::S85,
          _ => State::DEAD,
        }
      }
      State::S64 => {
        match byte {
          b'o' => State::S86,
          _ => State::DEAD,
        }
      }
      State::S65 => {
        match byte {
          b'h' => State::S87,
          _ => State::DEAD,
        }
      }
      State::S66 => {
        return Some((Unit::Month, 3));
      }
      State::S67 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S88,
          b's' => State::S89,
        }
      }
      State::S68 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S90,
          b's' => State::S91,
        }
      }
      State::S69 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S92,
          b's' => State::S93,
        }
      }
      State::S70 => {
        return Some((Unit::Second, 3));
      }
      State::S71 => {
        match byte {
          b'n' => State::S94,
          _ => State::DEAD,
        }
      }
      State::S72 => {
        match byte {
          b'\x00'..=b'\xff' => State::S95,
        }
      }
      State::S73 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S96,
          b's' => State::S97,
        }
      }
      State::S74 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S98,
          b's' => State::S99,
        }
      }
      State::S75 => {
        return Some((Unit::Week, 3));
      }
      State::S76 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S100,
          b's' => State::S101,
        }
      }
      State::S77 => {
        return Some((Unit::Year, 3));
      }
      State::S78 => {
        return Some((Unit::Microsecond, 3));
      }
      State::S79 => {
        match byte {
          b'c' => State::S102,
          _ => State::DEAD,
        }
      }
      State::S80 => {
        return Some((Unit::Day, 4));
      }
      State::S81 => {
        return Some((Unit::Hour, 4));
      }
      State::S82 => {
        match byte {
          b'\x00'..=b'\xff' => State::S103,
        }
      }
      State::S83 => {
        return Some((Unit::Minute, 4));
      }
      State::S84 => {
        match byte {
          b'e' => State::S104,
          _ => State::DEAD,
        }
      }
      State::S85 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S105,
          b's' => State::S106,
        }
      }
      State::S86 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S107,
          b's' => State::S108,
        }
      }
      State::S87 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S109,
          b's' => State::S110,
        }
      }
      State::S88 => {
        return Some((Unit::Millisecond, 4));
      }
      State::S89 => {
        match byte {
          b'\x00'..=b'\xff' => State::S111,
        }
      }
      State::S90 => {
        return Some((Unit::Nanosecond, 4));
      }
      State::S91 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'\xff' => State::S112,
          b'e' => State::S113,
        }
      }
      State::S92 => {
        return Some((Unit::Nanosecond, 4));
      }
      State::S93 => {
        match byte {
          b'\x00'..=b'\xff' => State::S114,
        }
      }
      State::S94 => {
        match byte {
          b'd' => State::S115,
          _ => State::DEAD,
        }
      }
      State::S95 => {
        return Some((Unit::Second, 4));
      }
      State::S96 => {
        return Some((Unit::Microsecond, 4));
      }
      State::S97 => {
        match byte {
          b'\x00'..=b'\xff' => State::S116,
        }
      }
      State::S98 => {
        return Some((Unit::Week, 4));
      }
      State::S99 => {
        match byte {
          b'\x00'..=b'\xff' => State::S117,
        }
      }
      State::S100 => {
        return Some((Unit::Year, 4));
      }
      State::S101 => {
        match byte {
          b'\x00'..=b'\xff' => State::S118,
        }
      }
      State::S102 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S119,
          b's' => State::S120,
        }
      }
      State::S103 => {
        return Some((Unit::Hour, 5));
      }
      State::S104 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S121,
          b's' => State::S122,
        }
      }
      State::S105 => {
        return Some((Unit::Millisecond, 5));
      }
      State::S106 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'\xff' => State::S123,
          b'e' => State::S124,
        }
      }
      State::S107 => {
        return Some((Unit::Microsecond, 5));
      }
      State::S108 => {
        match byte {
          b'\x00'..=b'd' | b'f'..=b'\xff' => State::S125,
          b'e' => State::S126,
        }
      }
      State::S109 => {
        return Some((Unit::Month, 5));
      }
      State::S110 => {
        match byte {
          b'\x00'..=b'\xff' => State::S127,
        }
      }
      State::S111 => {
        return Some((Unit::Millisecond, 5));
      }
      State::S112 => {
        return Some((Unit::Nanosecond, 5));
      }
      State::S113 => {
        match byte {
          b'c' => State::S128,
          _ => State::DEAD,
        }
      }
      State::S114 => {
        return Some((Unit::Nanosecond, 5));
      }
      State::S115 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S129,
          b's' => State::S130,
        }
      }
      State::S116 => {
        return Some((Unit::Microsecond, 5));
      }
      State::S117 => {
        return Some((Unit::Week, 5));
      }
      State::S118 => {
        return Some((Unit::Year, 5));
      }
      State::S119 => {
        return Some((Unit::Microsecond, 5));
      }
      State::S120 => {
        match byte {
          b'\x00'..=b'\xff' => State::S131,
        }
      }
      State::S121 => {
        return Some((Unit::Minute, 6));
      }
      State::S122 => {
        match byte {
          b'\x00'..=b'\xff' => State::S132,
        }
      }
      State::S123 => {
        return Some((Unit::Millisecond, 6));
      }
      State::S124 => {
        match byte {
          b'c' => State::S133,
          _ => State::DEAD,
        }
      }
      State::S125 => {
        return Some((Unit::Microsecond, 6));
      }
      State::S126 => {
        match byte {
          b'c' => State::S134,
          _ => State::DEAD,
        }
      }
      State::S127 => {
        return Some((Unit::Month, 6));
      }
      State::S128 => {
        match byte {
          b'o' => State::S135,
          _ => State::DEAD,
        }
      }
      State::S129 => {
        return Some((Unit::Second, 6));
      }
      State::S130 => {
        match byte {
          b'\x00'..=b'\xff' => State::S136,
        }
      }
      State::S131 => {
        return Some((Unit::Microsecond, 6));
      }
      State::S132 => {
        return Some((Unit::Minute, 7));
      }
      State::S133 => {
        match byte {
          b'o' => State::S137,
          _ => State::DEAD,
        }
      }
      State::S134 => {
        match byte {
          b'o' => State::S138,
          _ => State::DEAD,
        }
      }
      State::S135 => {
        match byte {
          b'n' => State::S139,
          _ => State::DEAD,
        }
      }
      State::S136 => {
        return Some((Unit::Second, 7));
      }
      State::S137 => {
        match byte {
          b'n' => State::S140,
          _ => State::DEAD,
        }
      }
      State::S138 => {
        match byte {
          b'n' => State::S141,
          _ => State::DEAD,
        }
      }
      State::S139 => {
        match byte {
          b'd' => State::S142,
          _ => State::DEAD,
        }
      }
      State::S140 => {
        match byte {
          b'd' => State::S143,
          _ => State::DEAD,
        }
      }
      State::S141 => {
        match byte {
          b'd' => State::S144,
          _ => State::DEAD,
        }
      }
      State::S142 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S145,
          b's' => State::S146,
        }
      }
      State::S143 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S147,
          b's' => State::S148,
        }
      }
      State::S144 => {
        match byte {
          b'\x00'..=b'r' | b't'..=b'\xff' => State::S149,
          b's' => State::S150,
        }
      }
      State::S145 => {
        return Some((Unit::Nanosecond, 10));
      }
      State::S146 => {
        match byte {
          b'\x00'..=b'\xff' => State::S151,
        }
      }
      State::S147 => {
        return Some((Unit::Millisecond, 11));
      }
      State::S148 => {
        match byte {
          b'\x00'..=b'\xff' => State::S152,
        }
      }
      State::S149 => {
        return Some((Unit::Microsecond, 11));
      }
      State::S150 => {
        match byte {
          b'\x00'..=b'\xff' => State::S153,
        }
      }
      State::S151 => {
        return Some((Unit::Nanosecond, 11));
      }
      State::S152 => {
        return Some((Unit::Millisecond, 12));
      }
      State::S153 => {
        return Some((Unit::Microsecond, 12));
      }
    };
  }
  return match sid {
    State::S1 => Some((Unit::Day, 1)),
    State::S2 => Some((Unit::Hour, 1)),
    State::S3 => Some((Unit::Minute, 1)),
    State::S5 => Some((Unit::Second, 1)),
    State::S7 => Some((Unit::Week, 1)),
    State::S8 => Some((Unit::Year, 1)),
    State::S14 => Some((Unit::Hour, 2)),
    State::S17 => Some((Unit::Month, 2)),
    State::S18 => Some((Unit::Millisecond, 2)),
    State::S20 => Some((Unit::Nanosecond, 2)),
    State::S23 => Some((Unit::Microsecond, 2)),
    State::S26 => Some((Unit::Week, 2)),
    State::S29 => Some((Unit::Year, 2)),
    State::S31 => Some((Unit::Day, 3)),
    State::S34 => Some((Unit::Hour, 3)),
    State::S35 => Some((Unit::Minute, 3)),
    State::S40 => Some((Unit::Month, 3)),
    State::S46 => Some((Unit::Second, 3)),
    State::S51 => Some((Unit::Week, 3)),
    State::S54 => Some((Unit::Year, 3)),
    State::S55 => Some((Unit::Microsecond, 3)),
    State::S57 => Some((Unit::Day, 4)),
    State::S58 => Some((Unit::Hour, 4)),
    State::S61 => Some((Unit::Minute, 4)),
    State::S67 => Some((Unit::Millisecond, 4)),
    State::S68 => Some((Unit::Nanosecond, 4)),
    State::S69 => Some((Unit::Nanosecond, 4)),
    State::S72 => Some((Unit::Second, 4)),
    State::S73 => Some((Unit::Microsecond, 4)),
    State::S74 => Some((Unit::Week, 4)),
    State::S76 => Some((Unit::Year, 4)),
    State::S82 => Some((Unit::Hour, 5)),
    State::S85 => Some((Unit::Millisecond, 5)),
    State::S86 => Some((Unit::Microsecond, 5)),
    State::S87 => Some((Unit::Month, 5)),
    State::S89 => Some((Unit::Millisecond, 5)),
    State::S91 => Some((Unit::Nanosecond, 5)),
    State::S93 => Some((Unit::Nanosecond, 5)),
    State::S97 => Some((Unit::Microsecond, 5)),
    State::S99 => Some((Unit::Week, 5)),
    State::S101 => Some((Unit::Year, 5)),
    State::S102 => Some((Unit::Microsecond, 5)),
    State::S104 => Some((Unit::Minute, 6)),
    State::S106 => Some((Unit::Millisecond, 6)),
    State::S108 => Some((Unit::Microsecond, 6)),
    State::S110 => Some((Unit::Month, 6)),
    State::S115 => Some((Unit::Second, 6)),
    State::S120 => Some((Unit::Microsecond, 6)),
    State::S122 => Some((Unit::Minute, 7)),
    State::S130 => Some((Unit::Second, 7)),
    State::S142 => Some((Unit::Nanosecond, 10)),
    State::S143 => Some((Unit::Millisecond, 11)),
    State::S144 => Some((Unit::Microsecond, 11)),
    State::S146 => Some((Unit::Nanosecond, 11)),
    State::S148 => Some((Unit::Millisecond, 12)),
    State::S150 => Some((Unit::Microsecond, 12)),
    _ => None,
  };

  enum State {
    DEAD,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
    S16,
    S17,
    S18,
    S19,
    S20,
    S21,
    S22,
    S23,
    S24,
    S25,
    S26,
    S27,
    S28,
    S29,
    S30,
    S31,
    S32,
    S33,
    S34,
    S35,
    S36,
    S37,
    S38,
    S39,
    S40,
    S41,
    S42,
    S43,
    S44,
    S45,
    S46,
    S47,
    S48,
    S49,
    S50,
    S51,
    S52,
    S53,
    S54,
    S55,
    S56,
    S57,
    S58,
    S59,
    S60,
    S61,
    S62,
    S63,
    S64,
    S65,
    S66,
    S67,
    S68,
    S69,
    S70,
    S71,
    S72,
    S73,
    S74,
    S75,
    S76,
    S77,
    S78,
    S79,
    S80,
    S81,
    S82,
    S83,
    S84,
    S85,
    S86,
    S87,
    S88,
    S89,
    S90,
    S91,
    S92,
    S93,
    S94,
    S95,
    S96,
    S97,
    S98,
    S99,
    S100,
    S101,
    S102,
    S103,
    S104,
    S105,
    S106,
    S107,
    S108,
    S109,
    S110,
    S111,
    S112,
    S113,
    S114,
    S115,
    S116,
    S117,
    S118,
    S119,
    S120,
    S121,
    S122,
    S123,
    S124,
    S125,
    S126,
    S127,
    S128,
    S129,
    S130,
    S131,
    S132,
    S133,
    S134,
    S135,
    S136,
    S137,
    S138,
    S139,
    S140,
    S141,
    S142,
    S143,
    S144,
    S145,
    S146,
    S147,
    S148,
    S149,
    S150,
    S151,
    S152,
    S153,
  }
}
