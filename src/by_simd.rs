use core::arch::x86_64::*;

use crate::Unit;

const SEED: u64 = 0xb8e624b1299dcf7c;

#[target_feature(enable = "avx512bw", enable = "avx512vl", enable = "avx512f")]
pub unsafe fn lookup_unsafe<'i>(input: &'i [u8]) -> Result<(Unit, &'i [u8]), anyhow::Error> {
        let mask = (1 << input.len().min(16)) - 1;
        let k = _cvtu32_mask16(mask);
        let inp = _mm_maskz_loadu_epi8(k, input.as_ptr() as *const i8);
        let lcalpha1 = _mm_cmp_epu8_mask::<_MM_CMPINT_NLT>(inp, _mm_set1_epi8(b'a' as i8));
        let lcalpha2 = _mm_cmp_epu8_mask::<_MM_CMPINT_LE>(inp, _mm_set1_epi8(b'z' as i8));
        let lcalpha = _kand_mask16(lcalpha1, lcalpha2);
        let mu1 = _mm_cmp_epu8_mask::<_MM_CMPINT_EQ>(inp, _mm_set1_epi8(0xc2u8 as i8));
        let mu2 = _mm_cmp_epu8_mask::<_MM_CMPINT_EQ>(inp, _mm_set1_epi8(0xb5u8 as i8));
        let mu = _kor_mask16(mu1, mu2);
        let valid = _kor_mask16(lcalpha, mu);
        let len_valid = _cvtmask16_u32(valid).trailing_ones();
        let valid_mask = _cvtu32_mask16(_bzhi_u32(_cvtmask16_u32(valid), len_valid));
        let inp_masked = _mm_maskz_mov_epi8(valid_mask, inp);
        let x = _mm_cvtsi128_si64(inp_masked) as u64;
        let hash = x.wrapping_add(len_valid as u64).wrapping_mul(SEED) >> (64 - 7);
        let probe = _mm_loadu_epi64(STRINGS.as_ptr().add(hash as usize) as *const i64);
        let eq = _mm_cmp_epu8_mask::<_MM_CMPINT_NE>(inp_masked, probe);
        if _ktestz_mask16_u8(eq, eq) != 0 {
            let unit = UNITS[hash as usize];
            Ok((unit, &input[len_valid as usize..]))
        } else {
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
        }
}

#[cfg(all(target_feature = "avx512bw", target_feature = "avx512vl", target_feature = "avx512f"))]
#[inline(never)]
pub fn lookup<'i>(input: &'i [u8]) -> Result<(Unit, &'i [u8]), anyhow::Error> {
    unsafe {
        lookup_unsafe(input)
    }
}

static STRINGS: [[u64; 2]; 128] = [
    [0x0, 0x0],
    [0x0, 0x0],
    [0x696c6c696d, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x7268, 0x0],
    [0x6e696d, 0x0],
    [0x636573696c6c696d, 0x646e6f],
    [0x0, 0x0],
    [0x68746e6f6d, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x79, 0x0],
    [0x6f6e616e, 0x0],
    [0x736365736e, 0x0],
    [0x6f6365736f6e616e, 0x73646e],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x6b656577, 0x0],
    [0x736f6d, 0x0],
    [0x0, 0x0],
    [0x7363657375, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x636573, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x736574756e696d, 0x0],
    [0x73796164, 0x0],
    [0x737268, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x736e, 0x0],
    [0x0, 0x0],
    [0x7372616579, 0x0],
    [0x0, 0x0],
    [0x6574756e696d, 0x0],
    [0x0, 0x0],
    [0x7279, 0x0],
    [0x7375, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x736e696d, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x736365736d, 0x0],
    [0x0, 0x0],
    [0x6f6365736f6e616e, 0x646e],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x6d, 0x0],
    [0x72756f68, 0x0],
    [0x0, 0x0],
    [0x736f7263696d, 0x0],
    [0x0, 0x0],
    [0x73b5c2, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x73636573, 0x0],
    [0x737279, 0x0],
    [0x73636573b5c2, 0x0],
    [0x736f6e616e, 0x0],
    [0x0, 0x0],
    [0x736d, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x736b656577, 0x0],
    [0x6b77, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x6365736f7263696d, 0x73646e6f],
    [0x0, 0x0],
    [0x73696c6c696d, 0x0],
    [0x0, 0x0],
    [0x73646e6f636573, 0x0],
    [0x0, 0x0],
    [0x77, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x7368746e6f6d, 0x0],
    [0x0, 0x0],
    [0x6365736e, 0x0],
    [0x0, 0x0],
    [0x646e6f636573, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x63657375, 0x0],
    [0x636573696c6c696d, 0x73646e6f],
    [0x73, 0x0],
    [0x736b77, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x68, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x6f7263696d, 0x0],
    [0x7372756f68, 0x0],
    [0x72616579, 0x0],
    [0x0, 0x0],
    [0x6365736f7263696d, 0x646e6f],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x636573b5c2, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x796164, 0x0],
    [0x64, 0x0],
    [0x6f6d, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x0, 0x0],
    [0x6365736d, 0x0],
    [0x0, 0x0],
];
static UNITS: [Unit; 128] = [
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Millisecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Hour,
    Unit::Minute,
    Unit::Millisecond,
    Unit::Nanosecond,
    Unit::Month,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Year,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Week,
    Unit::Month,
    Unit::Nanosecond,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Second,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Minute,
    Unit::Day,
    Unit::Hour,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Year,
    Unit::Nanosecond,
    Unit::Minute,
    Unit::Nanosecond,
    Unit::Year,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Minute,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Millisecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Minute,
    Unit::Hour,
    Unit::Nanosecond,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Second,
    Unit::Year,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Millisecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Week,
    Unit::Week,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Millisecond,
    Unit::Nanosecond,
    Unit::Second,
    Unit::Nanosecond,
    Unit::Week,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Month,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Second,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Microsecond,
    Unit::Millisecond,
    Unit::Second,
    Unit::Week,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Hour,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Microsecond,
    Unit::Hour,
    Unit::Year,
    Unit::Nanosecond,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Microsecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Day,
    Unit::Day,
    Unit::Month,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Nanosecond,
    Unit::Millisecond,
    Unit::Nanosecond,
];

#[test]
fn simd() {
    let (unit, remaining) = lookup(b"y 5 months").unwrap();
    assert_eq!(unit, Unit::Year);
    assert_eq!(remaining, b" 5 months");
}