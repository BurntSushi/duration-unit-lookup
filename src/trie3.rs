// This is like trie2, but very slightly tweaks how we find the unit value
// associated with the matching state. Instead of extracting it every time we
// see a match, we just store the matching node ID.
//
// This was a "simple" attempt at fixing trie2 after profiling revealed that a
// lot of time was being spent accessing the match state values. This was an
// attempt to do something easy before more invasive changes (see trie4).

type TrieNodeId = u8;

#[derive(Clone, Debug)]
pub(crate) struct Trie<
    const NODE_CAPACITY: usize,
    const ALPHABET_LEN: usize,
    V: 'static,
> {
    nodes: [TrieNode<ALPHABET_LEN, V>; NODE_CAPACITY],
    len: usize,
    alphabet: TrieAlphabet,
    max: usize,
}

impl<
        const NODE_CAPACITY: usize,
        const ALPHABET_LEN: usize,
        V: Copy + 'static,
    > Trie<NODE_CAPACITY, ALPHABET_LEN, V>
{
    const FAIL_ID: TrieNodeId = 0;
    const ROOT_ID: TrieNodeId = 1;

    pub(crate) const fn new(
        needles: &TrieNeedles<V>,
    ) -> Trie<NODE_CAPACITY, ALPHABET_LEN, V> {
        let mut trie = Trie {
            nodes: [TrieNode {
                transitions: [Self::FAIL_ID; ALPHABET_LEN],
                value: None,
            }; NODE_CAPACITY],
            len: 2,
            alphabet: needles.alphabet(),
            max: 0,
        };
        // The alphabet length inferred by the caller need to match the actual
        // alphabet length of the trie.
        assert!(trie.alphabet.len() == ALPHABET_LEN);
        // It must be impossible to exceed our identifier capacity.
        assert!(NODE_CAPACITY <= TrieNodeId::MAX as usize);

        let mut i = 0;
        while i < needles.map.len() {
            let (needle, unit) = needles.map[i];
            i += 1;
            if needle.len() > trie.max {
                trie.max = needle.len();
            }

            let mut node_id = Self::ROOT_ID;
            let mut k = 0;
            while k < needle.len() {
                let byte = needle.as_bytes()[k];
                k += 1;

                // MSRV(1.83): Use `trie.next` here instead of manually
                // inlining it. At present, it's forbidden because rustc thinks
                // it could result in a borrow containing interior mutability,
                // which I think is rooted in `V` being generic.
                let mut next_id = trie.nodes[node_id as usize].transitions
                    [trie.alphabet.equiv_id(byte) as usize];
                if next_id == Self::FAIL_ID {
                    // If this assertion fails, then the Trie needs more
                    // capacity.
                    assert!(trie.len < NODE_CAPACITY);
                    next_id = trie.len as u8;
                    trie.len += 1;
                }
                let equiv_id = trie.alphabet.equiv_id(byte);
                trie.nodes[node_id as usize].transitions[equiv_id as usize] =
                    next_id;
                node_id = next_id;
            }
            // This assert prevents duplicate needles. That is, every needle
            // must map to one and precisely one value. We could support
            // "overwrite" semantics, but I think it's better to fail loudly
            // here.
            if let Some(_) = trie.nodes[node_id as usize].value {
                panic!("duplicate needle detected");
            }
            trie.nodes[node_id as usize].value = Some(unit);
        }
        trie
    }

    #[inline(always)]
    pub(crate) fn find<'h>(&self, haystack: &'h [u8]) -> Option<(V, usize)> {
        let mut node_id = Self::ROOT_ID;
        let mut found = if self.nodes[node_id as usize].value.is_some() {
            Some((node_id, 0))
        } else {
            None
        };
        for (i, &byte) in haystack.iter().enumerate() {
            node_id = self.next(node_id, byte);
            if node_id == Self::FAIL_ID {
                break;
            }
            if self.nodes[node_id as usize].value.is_some() {
                found = Some((node_id, i + 1));
            }
        }
        found.map(|(node_id, offset)| {
            (self.nodes[node_id as usize].value.unwrap(), offset)
        })
    }

    #[inline(always)]
    const fn next(&self, current_id: TrieNodeId, byte: u8) -> TrieNodeId {
        let equiv_id = self.alphabet.equiv_id(byte);
        self.nodes[current_id as usize].transitions[equiv_id as usize]
    }
}

#[derive(Clone, Copy, Debug)]
struct TrieNode<const ALPHABET_LEN: usize, V: 'static> {
    transitions: [TrieNodeId; ALPHABET_LEN],
    value: Option<V>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TrieNeedles<V: 'static> {
    map: &'static [(&'static str, V)],
}

impl<V: Copy + 'static> TrieNeedles<V> {
    pub(crate) const fn new(
        map: &'static [(&'static str, V)],
    ) -> TrieNeedles<V> {
        TrieNeedles { map }
    }

    pub(crate) const fn alphabet_len(&self) -> usize {
        self.alphabet().len()
    }

    const fn alphabet(&self) -> TrieAlphabet {
        TrieAlphabet::new(self)
    }
}

#[derive(Clone)]
struct TrieAlphabet {
    len: u16,
    equiv_classes: [u8; 256],
}

impl TrieAlphabet {
    const fn new<V: Copy>(needles: &TrieNeedles<V>) -> TrieAlphabet {
        let mut equiv_set = [false; 256];
        let mut i = 0;
        while i < needles.map.len() {
            let (needle, _) = needles.map[i];
            i += 1;

            let mut k = 0;
            while k < needle.len() {
                let byte = needle.as_bytes()[k];
                equiv_set[byte as usize] = true;
                k += 1;
            }
        }

        // Count the number of distinct bytes seen in the needles. If we get
        // to 256, then that's the number of equivalence classes since there
        // are no byte values that don't occur in the needles. But if we get
        // N<256, then the number of equivalence classes is N+1, where the +1
        // accounts for the equivalence class containing all byte values that
        // were not present in the needles.
        let mut len = 0;
        let mut i = 0;
        while i < equiv_set.len() {
            if equiv_set[i] {
                len += 1;
            }
            i += 1;
        }
        if len < 256 {
            len += 1;
        }

        let mut equiv_classes = [0x00; 256];
        let mut byte: u16 = 0;
        if len == 256 {
            // In this case, there are 256 distinct bytes present in the
            // needles. So there must be exactly 256 equivalence classes and
            // each class just maps to the corresponding byte value.
            while byte < 256 {
                equiv_classes[byte as usize] = byte as u8;
                byte += 1;
            }
        } else {
            // We start at 1 here since we know there must be at least some
            // bytes that are not in the alphabet, and all of those get
            // implicitly assigned to equivalent class identifier `0`.
            let mut equiv_id: u16 = 1;
            while byte < 256 {
                if equiv_set[byte as usize] {
                    // Correct because we are limited to 256 iterations. We do
                    // start at 1, which means we could get to 256 here (before
                    // incrementing to 257 below), but that would require
                    // 256 distinct bytes. And that case is handled above.
                    assert!(equiv_id < 256);
                    equiv_classes[byte as usize] = equiv_id as u8;
                    equiv_id += 1;
                }
                byte += 1;
            }
        }
        TrieAlphabet { len, equiv_classes }
    }

    #[inline(always)]
    const fn len(&self) -> usize {
        self.len as usize
    }

    #[inline(always)]
    const fn equiv_id(&self, byte: u8) -> u8 {
        self.equiv_classes[byte as usize]
    }
}

impl core::fmt::Debug for TrieAlphabet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrieAlphabet").field("len", &self.len).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        type EmptyTrie = Trie<2, { EMPTY_NEEDLES.alphabet_len() }, char>;
        static EMPTY_TRIE: &'static EmptyTrie = &Trie::new(&EMPTY_NEEDLES);
        const EMPTY_NEEDLES: TrieNeedles<char> =
            TrieNeedles::new(&[("", 'z')]);

        assert_eq!(EMPTY_TRIE.find(b""), Some(('z', 0)));
    }

    #[test]
    fn fubar() {
        type FooTrie = Trie<12, { FOO_NEEDLES.alphabet_len() }, char>;
        static FOO_TRIE: &'static FooTrie = &Trie::new(&FOO_NEEDLES);
        const FOO_NEEDLES: TrieNeedles<char> =
            TrieNeedles::new(&[("foo", 'a'), ("bar", 'b'), ("quux", 'c')]);

        assert_eq!(FOO_TRIE.find(b""), None);
        assert_eq!(FOO_TRIE.find(b"fo"), None);
        assert_eq!(FOO_TRIE.find(b"foo"), Some(('a', 3)));
        assert_eq!(FOO_TRIE.find(b"fooquux"), Some(('a', 3)));
    }

    #[test]
    fn aaa() {
        type AaaTrie = Trie<12, { AAA_NEEDLES.alphabet_len() }, char>;
        static AAA_TRIE: &'static AaaTrie = &Trie::new(&AAA_NEEDLES);
        const AAA_NEEDLES: TrieNeedles<char> =
            TrieNeedles::new(&[("a", 'a'), ("aa", 'b'), ("aaa", 'c')]);

        assert_eq!(AAA_TRIE.find(b""), None);
        assert_eq!(AAA_TRIE.find(b"a"), Some(('a', 1)));
        assert_eq!(AAA_TRIE.find(b"aa"), Some(('b', 2)));
        assert_eq!(AAA_TRIE.find(b"aaa"), Some(('c', 3)));
        assert_eq!(AAA_TRIE.find(b"aaaa"), Some(('c', 3)));
    }
}
