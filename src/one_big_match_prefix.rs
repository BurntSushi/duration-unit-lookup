use crate::{one_big_match_prefix_gen::find, Unit};

#[inline(never)]
pub fn lookup<'i>(input: &'i [u8]) -> Result<(Unit, &'i [u8]), anyhow::Error> {
    let Some((unit, len)) = find(input) else {
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
    Ok((unit, &input[len..]))
}
