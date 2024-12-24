use crate::Unit;

#[repr(C)]
struct Output {
    unit: Unit,
    len: usize,
}

extern "C" {
    fn gencdfa1_find(start: *const u8, end: *const u8) -> Output;
}

#[inline(never)]
pub fn lookup<'i>(input: &'i [u8]) -> Result<(Unit, &'i [u8]), anyhow::Error> {
    let p = input.as_ptr();
    let o = unsafe { gencdfa1_find(p, p.add(input.len())) };
    if o.len == 0 {
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
    Ok((o.unit, &input[o.len..]))
}
