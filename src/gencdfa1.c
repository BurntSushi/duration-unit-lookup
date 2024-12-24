#include <stddef.h>
#include <stdint.h>

enum unit {
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
};

struct output {
  enum unit unit;
  size_t length;
};

struct output gencdfa1_find(uint8_t *p, uint8_t *end)
{
  struct output o = { .unit = Year, .length = 0 };
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'd': goto S1;
    case 'h': goto S2;
    case 'm': goto S3;
    case 'n': goto S4;
    case 's': goto S5;
    case 'u': goto S6;
    case 'w': goto S7;
    case 'y': goto S8;
    case 0xc2: goto S9;
    default: goto DONE;
  }
S1:
  if (p >= end) {
    goto S10;
  }
  switch (*p++) {
    case 0x00 ... 0x60: goto S10;
    case 'b' ... 0xff: goto S10;
    case 'a': goto S11;
  }
S2:
  if (p >= end) {
    goto S12;
  }
  switch (*p++) {
    case 0x00 ... 'n': goto S12;
    case 'p' ... 'q': goto S12;
    case 's' ... 0xff: goto S12;
    case 'o': goto S13;
    case 'r': goto S14;
  }
S3:
  if (p >= end) {
    goto S15;
  }
  switch (*p++) {
    case 0x00 ... 'h': goto S15;
    case 'j' ... 'n': goto S15;
    case 'p' ... 'r': goto S15;
    case 't' ... 0xff: goto S15;
    case 'i': goto S16;
    case 'o': goto S17;
    case 's': goto S18;
  }
S4:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'a': goto S19;
    case 's': goto S20;
    default: goto DONE;
  }
S5:
  if (p >= end) {
    goto S21;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S21;
    case 'f' ... 0xff: goto S21;
    case 'e': goto S22;
  }
S6:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 's': goto S23;
    default: goto DONE;
  }
S7:
  if (p >= end) {
    goto S24;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S24;
    case 'f' ... 'j': goto S24;
    case 'l' ... 0xff: goto S24;
    case 'e': goto S25;
    case 'k': goto S26;
  }
S8:
  if (p >= end) {
    goto S27;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S27;
    case 'f' ... 'q': goto S27;
    case 's' ... 0xff: goto S27;
    case 'e': goto S28;
    case 'r': goto S29;
  }
S9:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 0xb5: goto S30;
    default: goto DONE;
  }
S10:
  o.unit = Day;
  o.length = 1;
  goto DONE;
S11:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'y': goto S31;
    default: goto DONE;
  }
S12:
  o.unit = Hour;
  o.length = 1;
  goto DONE;
S13:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'u': goto S32;
    default: goto DONE;
  }
S14:
  if (p >= end) {
    goto S33;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S33;
    case 't' ... 0xff: goto S33;
    case 's': goto S34;
  }
S15:
  o.unit = Minute;
  o.length = 1;
  goto DONE;
S16:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'n': goto S35;
    case 'l': goto S36;
    case 'c': goto S37;
    default: goto DONE;
  }
S17:
  if (p >= end) {
    goto S38;
  }
  switch (*p++) {
    case 0x00 ... 'm': goto S38;
    case 'o' ... 'r': goto S38;
    case 't' ... 0xff: goto S38;
    case 'n': goto S39;
    case 's': goto S40;
  }
S18:
  if (p >= end) {
    goto S41;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S41;
    case 'f' ... 0xff: goto S41;
    case 'e': goto S42;
  }
S19:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'n': goto S43;
    default: goto DONE;
  }
S20:
  if (p >= end) {
    goto S44;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S44;
    case 'f' ... 0xff: goto S44;
    case 'e': goto S45;
  }
S21:
  o.unit = Second;
  o.length = 1;
  goto DONE;
S22:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'c': goto S46;
    default: goto DONE;
  }
S23:
  if (p >= end) {
    goto S47;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S47;
    case 'f' ... 0xff: goto S47;
    case 'e': goto S48;
  }
S24:
  o.unit = Week;
  o.length = 1;
  goto DONE;
S25:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'e': goto S49;
    default: goto DONE;
  }
S26:
  if (p >= end) {
    goto S50;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S50;
    case 't' ... 0xff: goto S50;
    case 's': goto S51;
  }
S27:
  o.unit = Year;
  o.length = 1;
  goto DONE;
S28:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'a': goto S52;
    default: goto DONE;
  }
S29:
  if (p >= end) {
    goto S53;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S53;
    case 't' ... 0xff: goto S53;
    case 's': goto S54;
  }
S30:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 's': goto S55;
    default: goto DONE;
  }
S31:
  if (p >= end) {
    goto S56;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S56;
    case 't' ... 0xff: goto S56;
    case 's': goto S57;
  }
S32:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'r': goto S58;
    default: goto DONE;
  }
S33:
  o.unit = Hour;
  o.length = 2;
  goto DONE;
S34:
  o.unit = Hour;
  o.length = 3;
  goto DONE;
S35:
  if (p >= end) {
    goto S59;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S59;
    case 't': goto S59;
    case 'v' ... 0xff: goto S59;
    case 's': goto S60;
    case 'u': goto S61;
  }
S36:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'l': goto S62;
    default: goto DONE;
  }
S37:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'r': goto S63;
    default: goto DONE;
  }
S38:
  o.unit = Month;
  o.length = 2;
  goto DONE;
S39:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 't': goto S64;
    default: goto DONE;
  }
S40:
  o.unit = Month;
  o.length = 3;
  goto DONE;
S41:
  o.unit = Millisecond;
  o.length = 2;
  goto DONE;
S42:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'c': goto S65;
    default: goto DONE;
  }
S43:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'o': goto S66;
    default: goto DONE;
  }
S44:
  o.unit = Nanosecond;
  o.length = 2;
  goto DONE;
S45:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'c': goto S67;
    default: goto DONE;
  }
S46:
  if (p >= end) {
    goto S68;
  }
  switch (*p++) {
    case 0x00 ... 'n': goto S68;
    case 'p' ... 'r': goto S68;
    case 't' ... 0xff: goto S68;
    case 'o': goto S69;
    case 's': goto S70;
  }
S47:
  o.unit = Microsecond;
  o.length = 2;
  goto DONE;
S48:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'c': goto S71;
    default: goto DONE;
  }
S49:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'k': goto S72;
    default: goto DONE;
  }
S50:
  o.unit = Week;
  o.length = 2;
  goto DONE;
S51:
  o.unit = Week;
  o.length = 3;
  goto DONE;
S52:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'r': goto S73;
    default: goto DONE;
  }
S53:
  o.unit = Year;
  o.length = 2;
  goto DONE;
S54:
  o.unit = Year;
  o.length = 3;
  goto DONE;
S55:
  if (p >= end) {
    goto S74;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S74;
    case 'f' ... 0xff: goto S74;
    case 'e': goto S75;
  }
S56:
  o.unit = Day;
  o.length = 3;
  goto DONE;
S57:
  o.unit = Day;
  o.length = 4;
  goto DONE;
S58:
  if (p >= end) {
    goto S76;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S76;
    case 't' ... 0xff: goto S76;
    case 's': goto S77;
  }
S59:
  o.unit = Minute;
  o.length = 3;
  goto DONE;
S60:
  o.unit = Minute;
  o.length = 4;
  goto DONE;
S61:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 't': goto S78;
    default: goto DONE;
  }
S62:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'i': goto S79;
    default: goto DONE;
  }
S63:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'o': goto S80;
    default: goto DONE;
  }
S64:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'h': goto S81;
    default: goto DONE;
  }
S65:
  if (p >= end) {
    goto S82;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S82;
    case 't' ... 0xff: goto S82;
    case 's': goto S83;
  }
S66:
  if (p >= end) {
    goto S84;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S84;
    case 't' ... 0xff: goto S84;
    case 's': goto S85;
  }
S67:
  if (p >= end) {
    goto S86;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S86;
    case 't' ... 0xff: goto S86;
    case 's': goto S87;
  }
S68:
  o.unit = Second;
  o.length = 3;
  goto DONE;
S69:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'n': goto S88;
    default: goto DONE;
  }
S70:
  o.unit = Second;
  o.length = 4;
  goto DONE;
S71:
  if (p >= end) {
    goto S89;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S89;
    case 't' ... 0xff: goto S89;
    case 's': goto S90;
  }
S72:
  if (p >= end) {
    goto S91;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S91;
    case 't' ... 0xff: goto S91;
    case 's': goto S92;
  }
S73:
  if (p >= end) {
    goto S93;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S93;
    case 't' ... 0xff: goto S93;
    case 's': goto S94;
  }
S74:
  o.unit = Microsecond;
  o.length = 3;
  goto DONE;
S75:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'c': goto S95;
    default: goto DONE;
  }
S76:
  o.unit = Hour;
  o.length = 4;
  goto DONE;
S77:
  o.unit = Hour;
  o.length = 5;
  goto DONE;
S78:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'e': goto S96;
    default: goto DONE;
  }
S79:
  if (p >= end) {
    goto S97;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S97;
    case 't' ... 0xff: goto S97;
    case 's': goto S98;
  }
S80:
  if (p >= end) {
    goto S99;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S99;
    case 't' ... 0xff: goto S99;
    case 's': goto S100;
  }
S81:
  if (p >= end) {
    goto S101;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S101;
    case 't' ... 0xff: goto S101;
    case 's': goto S102;
  }
S82:
  o.unit = Millisecond;
  o.length = 4;
  goto DONE;
S83:
  o.unit = Millisecond;
  o.length = 5;
  goto DONE;
S84:
  o.unit = Nanosecond;
  o.length = 4;
  goto DONE;
S85:
  if (p >= end) {
    goto S103;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S103;
    case 'f' ... 0xff: goto S103;
    case 'e': goto S104;
  }
S86:
  o.unit = Nanosecond;
  o.length = 4;
  goto DONE;
S87:
  o.unit = Nanosecond;
  o.length = 5;
  goto DONE;
S88:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'd': goto S105;
    default: goto DONE;
  }
S89:
  o.unit = Microsecond;
  o.length = 4;
  goto DONE;
S90:
  o.unit = Microsecond;
  o.length = 5;
  goto DONE;
S91:
  o.unit = Week;
  o.length = 4;
  goto DONE;
S92:
  o.unit = Week;
  o.length = 5;
  goto DONE;
S93:
  o.unit = Year;
  o.length = 4;
  goto DONE;
S94:
  o.unit = Year;
  o.length = 5;
  goto DONE;
S95:
  if (p >= end) {
    goto S106;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S106;
    case 't' ... 0xff: goto S106;
    case 's': goto S107;
  }
S96:
  if (p >= end) {
    goto S108;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S108;
    case 't' ... 0xff: goto S108;
    case 's': goto S109;
  }
S97:
  o.unit = Millisecond;
  o.length = 5;
  goto DONE;
S98:
  if (p >= end) {
    goto S110;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S110;
    case 'f' ... 0xff: goto S110;
    case 'e': goto S111;
  }
S99:
  o.unit = Microsecond;
  o.length = 5;
  goto DONE;
S100:
  if (p >= end) {
    goto S112;
  }
  switch (*p++) {
    case 0x00 ... 'd': goto S112;
    case 'f' ... 0xff: goto S112;
    case 'e': goto S113;
  }
S101:
  o.unit = Month;
  o.length = 5;
  goto DONE;
S102:
  o.unit = Month;
  o.length = 6;
  goto DONE;
S103:
  o.unit = Nanosecond;
  o.length = 5;
  goto DONE;
S104:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'c': goto S114;
    default: goto DONE;
  }
S105:
  if (p >= end) {
    goto S115;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S115;
    case 't' ... 0xff: goto S115;
    case 's': goto S116;
  }
S106:
  o.unit = Microsecond;
  o.length = 5;
  goto DONE;
S107:
  o.unit = Microsecond;
  o.length = 6;
  goto DONE;
S108:
  o.unit = Minute;
  o.length = 6;
  goto DONE;
S109:
  o.unit = Minute;
  o.length = 7;
  goto DONE;
S110:
  o.unit = Millisecond;
  o.length = 6;
  goto DONE;
S111:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'c': goto S117;
    default: goto DONE;
  }
S112:
  o.unit = Microsecond;
  o.length = 6;
  goto DONE;
S113:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'c': goto S118;
    default: goto DONE;
  }
S114:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'o': goto S119;
    default: goto DONE;
  }
S115:
  o.unit = Second;
  o.length = 6;
  goto DONE;
S116:
  o.unit = Second;
  o.length = 7;
  goto DONE;
S117:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'o': goto S120;
    default: goto DONE;
  }
S118:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'o': goto S121;
    default: goto DONE;
  }
S119:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'n': goto S122;
    default: goto DONE;
  }
S120:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'n': goto S123;
    default: goto DONE;
  }
S121:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'n': goto S124;
    default: goto DONE;
  }
S122:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'd': goto S125;
    default: goto DONE;
  }
S123:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'd': goto S126;
    default: goto DONE;
  }
S124:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'd': goto S127;
    default: goto DONE;
  }
S125:
  if (p >= end) {
    goto S128;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S128;
    case 't' ... 0xff: goto S128;
    case 's': goto S129;
  }
S126:
  if (p >= end) {
    goto S130;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S130;
    case 't' ... 0xff: goto S130;
    case 's': goto S131;
  }
S127:
  if (p >= end) {
    goto S132;
  }
  switch (*p++) {
    case 0x00 ... 'r': goto S132;
    case 't' ... 0xff: goto S132;
    case 's': goto S133;
  }
S128:
  o.unit = Nanosecond;
  o.length = 10;
  goto DONE;
S129:
  o.unit = Nanosecond;
  o.length = 11;
  goto DONE;
S130:
  o.unit = Millisecond;
  o.length = 11;
  goto DONE;
S131:
  o.unit = Millisecond;
  o.length = 12;
  goto DONE;
S132:
  o.unit = Microsecond;
  o.length = 11;
  goto DONE;
S133:
  o.unit = Microsecond;
  o.length = 12;
  goto DONE;
DONE:
  return o;
}
