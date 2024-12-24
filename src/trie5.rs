// This was my final trie attempt. I copied that idea from one-big-match that
// maybe we could save time by finding the entire unit designator first and
// *then* do a lookup. This lets us simplify the trie find logic such that it
// doesn't need to quit its loop early. It just advances through the trie for
// every byte in the needle. And if the final state is matching, then we're
// good to go.
//
// This wasn't even noticeably faster than trie4, which is quite frustrating.
// It's a hair faster on the `short` and `medium` benchmarks, but slower on
// `long`. And of course, still slower than `one-big-match` (although faster on
// the `short` benchmark, which is interesting).

type TrieNodeId = u16;

#[derive(Clone, Debug)]
pub(crate) struct Trie<
    const TRANSITION_CAPACITY: usize,
    const NEEDLE_LEN: usize,
    const ALPHABET_LEN: usize,
    V: 'static,
> {
    transitions: [TrieNodeId; TRANSITION_CAPACITY],
    matches: [V; TRANSITION_CAPACITY],
    alphabet: TrieAlphabet,
    node_len: usize,
    max_needle_len: usize,
    max_match_node_id: TrieNodeId,
    root_node_id: TrieNodeId,
}

impl<
        const TRANSITION_CAPACITY: usize,
        const NEEDLE_LEN: usize,
        const ALPHABET_LEN: usize,
        V: Copy + 'static,
    > Trie<TRANSITION_CAPACITY, NEEDLE_LEN, ALPHABET_LEN, V>
{
    const FAIL_ID: TrieNodeId = 0;

    pub(crate) const fn new(
        needles: &TrieNeedles<V>,
        default: V,
    ) -> Trie<TRANSITION_CAPACITY, NEEDLE_LEN, ALPHABET_LEN, V> {
        let node_capacity = TRANSITION_CAPACITY / ALPHABET_LEN;
        // The alphabet length inferred by the caller need to match the actual
        // alphabet length of the trie.
        let alphabet = needles.alphabet();
        assert!(alphabet.len() == ALPHABET_LEN);
        // It must be impossible to exceed our identifier capacity.
        assert!(TRANSITION_CAPACITY <= TrieNodeId::MAX as usize);

        let mut trie: Trie<TRANSITION_CAPACITY, NEEDLE_LEN, ALPHABET_LEN, V> =
            Trie {
                transitions: [Self::FAIL_ID; TRANSITION_CAPACITY],
                matches: [default; TRANSITION_CAPACITY],
                alphabet,
                node_len: 2,
                max_needle_len: 0,
                max_match_node_id: Self::FAIL_ID,
                root_node_id: Self::FAIL_ID + ALPHABET_LEN as TrieNodeId,
            };
        // An intermediate array to store the transitions. This will basically
        // wind up being copied to `trie.transitions`, but with all match
        // nodes shuffled before all non-match nodes.
        let mut trans: [TrieNodeId; TRANSITION_CAPACITY] =
            [Self::FAIL_ID; TRANSITION_CAPACITY];
        // An intermediate array to accumulate the match values for each
        // match node. This is gratuitously wasteful, but we compact the
        // match values into `trie.matches` in a second pass.
        let mut matches: [Option<V>; TRANSITION_CAPACITY] =
            [None; TRANSITION_CAPACITY];

        let mut i = 0;
        while i < needles.map.len() {
            let (needle, unit) = needles.map[i];
            i += 1;
            if needle.len() > trie.max_needle_len {
                trie.max_needle_len = needle.len();
            }

            let mut node_id = trie.root_node_id;
            let mut k = 0;
            while k < needle.len() {
                let byte = needle.as_bytes()[k];
                let equiv_id = trie.alphabet.equiv_id(byte);
                k += 1;

                let mut next_id = trans[node_id as usize + equiv_id as usize];
                if next_id == Self::FAIL_ID {
                    // If this assertion fails, then the Trie needs more
                    // capacity.
                    assert!(trie.node_len < node_capacity);
                    // If this fails, then `TrieNodeId` needs to be a bigger
                    // primitive type.
                    next_id = (trie.node_len as TrieNodeId)
                        .checked_mul(ALPHABET_LEN as TrieNodeId)
                        .unwrap();
                    trie.node_len += 1;
                }
                trans[node_id as usize + equiv_id as usize] = next_id;
                node_id = next_id;
            }

            // This assert prevents duplicate needles. That is, every needle
            // must map to one and precisely one value. We could support
            // "overwrite" semantics, but I think it's better to fail loudly
            // here.
            if let Some(_) = matches[node_id as usize] {
                panic!("duplicate needle detected");
            }
            matches[node_id as usize] = Some(unit);
        }

        // First, copy over all match nodes to `trie.transitions`. That way,
        // all matching nodes will appear before non-matching nodes.
        //
        // Once exception to this is the fail node. That always comes first.
        // Because the case analysis in the search loop is:
        //
        //     if node_id <= max_match_node_id {
        //         if node_id == Self::FAIL_ID {
        //             break;
        //         }
        //         last_match = node_id;
        //     }
        //
        // That way, we have only one branch. In other words, the case analysis
        // is more like, "does this node need special attention?"
        //
        // This same optimization is used in regex-automata, but there are more
        // than fail, match and non-match nodes. There are other special nodes.
        let mut old_to_new: [TrieNodeId; TRANSITION_CAPACITY] =
            [Self::FAIL_ID; TRANSITION_CAPACITY];
        let mut old_node_index = 1;
        let mut new_node_index = 1;
        while old_node_index < trie.node_len {
            let old_node_id = old_node_index * (ALPHABET_LEN as usize);
            old_node_index += 1;
            let Some(match_value) = matches[old_node_id] else {
                continue;
            };

            let new_node_id = new_node_index * (ALPHABET_LEN as usize);
            new_node_index += 1;

            // Keep a record of the new node ID so that we can remap
            // transitions later.
            old_to_new[old_node_id] = new_node_id as TrieNodeId;
            // Copy the transitions over.
            let mut i = 0;
            while i < (ALPHABET_LEN as usize) {
                trie.transitions[new_node_id + i] = trans[old_node_id + i];
                i += 1;
            }
            // And record the match value.
            trie.matches[new_node_id] = match_value;
            // The last recording is the maximum node id that is a match
            // node, by construction.
            trie.max_match_node_id = new_node_id as TrieNodeId;
        }
        // Now do the same, but for non-match nodes.
        let mut old_node_index = 1;
        while old_node_index < trie.node_len {
            let old_node_id = old_node_index * (ALPHABET_LEN as usize);
            old_node_index += 1;
            if matches[old_node_id].is_some() {
                continue;
            }

            let new_node_id = new_node_index * (ALPHABET_LEN as usize);
            new_node_index += 1;

            // Keep a record of the new node ID so that we can remap
            // transitions later.
            old_to_new[old_node_id] = new_node_id as TrieNodeId;
            // Copy the transitions over.
            let mut i = 0;
            while i < (ALPHABET_LEN as usize) {
                trie.transitions[new_node_id + i] = trans[old_node_id + i];
                i += 1;
            }
        }

        // Re-map all transitions to reflect their new IDs.
        let mut i = 0;
        while i < trie.transitions.len() {
            trie.transitions[i] = old_to_new[trie.transitions[i] as usize];
            i += 1;
        }
        trie.root_node_id = old_to_new[trie.root_node_id as usize];

        trie
    }

    #[inline(always)]
    pub(crate) fn find<'h>(&self, haystack: &'h [u8]) -> Option<V> {
        let mut node_id = self.root_node_id;
        for &byte in haystack.iter() {
            let equiv_id = self.alphabet.equiv_id(byte);
            node_id = self.transitions[node_id as usize + equiv_id as usize];
        }
        if node_id <= self.max_match_node_id && node_id != Self::FAIL_ID {
            Some(self.matches[node_id as usize])
        } else {
            None
        }
    }
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

    pub(crate) const fn len(&self) -> usize {
        self.map.len()
    }

    pub(crate) const fn alphabet_len(&self) -> usize {
        self.alphabet().len()
    }

    pub(crate) const fn transition_len(&self, node_capacity: usize) -> usize {
        self.alphabet().len() * node_capacity
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
        type EmptyTrie = Trie<
            { EMPTY_NEEDLES.transition_len(2) },
            { EMPTY_NEEDLES.len() },
            { EMPTY_NEEDLES.alphabet_len() },
            char,
        >;
        static EMPTY_TRIE: &'static EmptyTrie =
            &Trie::new(&EMPTY_NEEDLES, '\0');
        const EMPTY_NEEDLES: TrieNeedles<char> =
            TrieNeedles::new(&[("", 'z')]);

        assert_eq!(EMPTY_TRIE.find(b""), Some('z'));
    }

    #[test]
    fn fubar() {
        type FooTrie = Trie<
            { FOO_NEEDLES.transition_len(12) },
            { FOO_NEEDLES.len() },
            { FOO_NEEDLES.alphabet_len() },
            char,
        >;
        const FOO_NEEDLES: TrieNeedles<char> =
            TrieNeedles::new(&[("foo", 'a'), ("bar", 'b'), ("quux", 'c')]);
        static FOO_TRIE: &'static FooTrie = &Trie::new(&FOO_NEEDLES, '\0');

        assert_eq!(FOO_TRIE.find(b""), None);
        assert_eq!(FOO_TRIE.find(b"fo"), None);
        assert_eq!(FOO_TRIE.find(b"foo"), Some('a'));
        assert_eq!(FOO_TRIE.find(b"quux"), Some('c'));
        assert_eq!(FOO_TRIE.find(b"fooquux"), None);
    }

    #[test]
    fn aaa() {
        type AaaTrie = Trie<
            { AAA_NEEDLES.transition_len(12) },
            { AAA_NEEDLES.len() },
            { AAA_NEEDLES.alphabet_len() },
            char,
        >;
        static AAA_TRIE: &'static AaaTrie = &Trie::new(&AAA_NEEDLES, '\0');
        const AAA_NEEDLES: TrieNeedles<char> =
            TrieNeedles::new(&[("a", 'a'), ("aa", 'b'), ("aaa", 'c')]);

        assert_eq!(AAA_TRIE.find(b""), None);
        assert_eq!(AAA_TRIE.find(b"a"), Some('a'));
        assert_eq!(AAA_TRIE.find(b"aa"), Some('b'));
        assert_eq!(AAA_TRIE.find(b"aaa"), Some('c'));
        assert_eq!(AAA_TRIE.find(b"aaaa"), None);
    }
}
