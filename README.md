Benchmarks for parsing human readable durations
===============================================
This repository measures a _number_ of different techniques for recognizing
a short string from a small set (around 50) of short strings.

This came up as part of the work in parsing a new duration format that
I was adding to [Jiff](https://github.com/BurntSushi/jiff). Specifically, I
wanted to support a "human friendly" duration format similar to what the
[`humantime`](https://docs.rs/humantime) crate supports. While Jiff had already
supported ISO 8601 durations (e.g., `P2Y1M15DT5H59M1S`), I found them somewhat
difficult to read and not as friendly as what `humantime` supports. Namely,
with `humantime`, you can reasonably ask end users to write a duration and it's
likely to parse. Here's an example of a duration that both Jiff and `humantime`
can parse (although they differ in interpretation!):

```
2 years 1 month 15 days 5 hours 59 minutes 1 second
```

Jiff supports a fair bit more than `humantime` and has a more configurable
printer. But I wanted to benchmark it with `humantime` to verify that Jiff's
implementation was at least competitive with `humantime`'s implementation.

One of the bigger bottlenecks in parsing durations like the above is
recognizing the unit designator labels like `years` and `minutes`. `humantime`
recognizes 35 different labels while Jiff recognizes 56.

When I built my first parser, I knew that `humantime` was using a `match`
statement to do this recognition. Specifically:

```rust
let (mut sec, nsec) = match &self.src[start..end] {
    "nanos" | "nsec" | "ns" => (0u64, n),
    "usec" | "us" => (0u64, n.mul(1000)?),
    "millis" | "msec" | "ms" => (0u64, n.mul(1_000_000)?),
    "seconds" | "second" | "secs" | "sec" | "s" => (n, 0),
    "minutes" | "minute" | "min" | "mins" | "m"
    => (n.mul(60)?, 0),
    "hours" | "hour" | "hr" | "hrs" | "h" => (n.mul(3600)?, 0),
    "days" | "day" | "d" => (n.mul(86400)?, 0),
    "weeks" | "week" | "w" => (n.mul(86400*7)?, 0),
    "months" | "month" | "M" => (n.mul(2_630_016)?, 0), // 30.44d
    "years" | "year" | "y" => (n.mul(31_557_600)?, 0), // 365.25d
    _ => {
        return Err(Error::UnknownUnit {
            start, end,
            unit: self.src[start..end].to_string(),
            value: n,
        });
    }
};
```

In order for this to work, you actually need to scan ahead to find the full
label, sub-slice it and then execute the `match` on the sub-slice. Why? Because
the unit designator label might be a strict prefix of the remaining input. So
for example, if you're trying to parse `5 hours 59 minutes`, then after parsing
`5` and the following whitespace, the remaining input is `hours 59 minutes`.
The `humantime` parser will scan the input to consume as many characters as is
possible until it sees a character that cannot be part of a valid label (like
whitespace). This would extract `hours`, and then you can `match` on it with
the set of supported labels and map this to some math that converts the parsed
integer into an actual duration.

With hubris, I thought I could easily beat this by cutting out the step that
first parses `hours`. I knew it was likely that the `match` expression itself
would be fast. But if I could avoid re-scanning the input and just recognizing
the label in a single pass, then surely this would be faster right?

So in my initial parser implementation, I decided to get very fancy
and implement a `const`-compatible trie. (You can find this at
[`src/trie1.rs`](src/trie1.rs).) Basically, at compile time, it generates a
finite state machine (as a table of transitions) from the supported set of unit
designator labels. And then provides a `find` function that traverses this
state machine based on the input. I didn't necessarily think it would be faster
than a `match`, but I thought it would for sure be faster than scanning for the
label _and_ a `match`. I thought this because the trie approach doesn't require
scanning for the needle ahead of time. You just traverse the trie using the
bytes in the input until you reach a state where no other match is possible. If
you found a match at any point, you return it.

When I went to benchmark my parser implementation against `humantime`, I was
surprised to discover that it was quite a bit slower! And indeed, a good chunk
of time was being spent recognizing the unit designator labels.

I thought that, for sure, my approach was still sound. And I just hadn't
optimized my trie enough. That's when I made this repository to try out
some different tricks I had employed in my experience working on
[`regex-automata`].

## Running and viewing the benchmarks

To run them:

```
cargo bench -- --save-baseline friendly
```

And to easily analyze them, consider using [`critcmp`]:

```
$ critcmp friendly -g '.*?/(.*)' -f 'by-trie5|phf|one-big-match|gendfa1|gencdfa1'
group     friendly/by-gencdfa1/                  friendly/by-gendfa1/                   friendly/by-trie5/                     friendly/one-big-match-prefix/         friendly/one-big-match/                friendly/phf/
-----     ---------------------                  --------------------                   ------------------                     ------------------------------         -----------------------                -------------
long      1.90      4.8±0.12ns        ? ?/sec    5.16     13.0±0.05ns        ? ?/sec    4.32     10.9±0.19ns        ? ?/sec    1.00      2.5±0.06ns        ? ?/sec    2.60      6.6±0.25ns        ? ?/sec    6.57     16.6±0.08ns        ? ?/sec
medium    1.77      3.5±0.11ns        ? ?/sec    3.99      7.9±0.03ns        ? ?/sec    3.16      6.2±0.09ns        ? ?/sec    1.00      2.0±0.06ns        ? ?/sec    2.12      4.2±0.09ns        ? ?/sec    6.85     13.6±0.09ns        ? ?/sec
short     1.00      2.6±0.05ns        ? ?/sec    1.46      3.9±0.05ns        ? ?/sec    1.03      2.7±0.07ns        ? ?/sec    1.25      3.3±0.09ns        ? ?/sec    1.10      2.9±0.05ns        ? ?/sec    4.02     10.6±0.07ns        ? ?/sec
```

Each technique in this repository has the same 3 benchmarks: parsing a short,
medium and long unit designator label. The short label is `y`. The medium label
is `months`. The long label is `milliseconds`. All inputs include additional
text after the unit designator label so that all of the techniques _must_
handle parsing a label as a strict prefix of some input.

Here are all of the techniques benchmarked in this repository:

* `one-big-match` is the `humantime` approach of scanning to find the full
label, and then executing a `match` on the discovered label to see if it
matches any of the known labels.
* `one-big-match-prefix` is like `one-big-match`, but matches on slice
prefixes.
* `aho-corasick` uses the [`aho-corasick`] crate to parse unit designator
labels.
* `phf` uses the `phf` crate for perfect hashing to parse unit designator
labels.
* `by-trie{1,2,3,4,5}` are 5 different trie implementations, each an evolution
of the previous one, attempting something different to be faster.
* `gendfa1` is Rust source code generated from a `regex-automata` DFA that
recognizes each of the unit designator labels.
* `gencdfa1` is like `gendfa1`, but generates C code instead. The advantage of
C is that it has `goto`, which makes encoding finite state machines very
straight-forward.

## Analysis

### Tries

I started by iterating on the initial version of my trie:

> **NOTE:** It's not terribly important to understand all of the trie details
> here. Just that I tried a bunch of different things based on my prior
> experience. While I did get some improvements, I never got it to be faster
> than the scan-and-`match` approach.)

* [`src/trie1.rs`](src/trie1.rs) is my initial implementation. I was so
confident that it would be superior that I even lovingly commented it before
even verifying that it was fast. Such hubris.
* [`src/trie2.rs`](src/trie2.rs) tweaks the return type to use an offset
instead of a slice.
* [`src/trie3.rs`](src/trie3.rs) does a small tweak in how we extract the
matching `Unit` corresponding to the label. I did this because profiling
suggested a lot of time was being spent accessing the match values in the
loop walking the trie during a search. Some inputs result in multiple matches,
so this can be a little costly. For example, `minutes` matches `m`, `min`,
`minute` and finally `minutes`. So this visits a match state 4 times. In this
trie, we switched to just keeping track of the matching node ID. But this was
a tiny tweak that didn't really change the data layout of the transition table.
* [`src/trie4.rs`](src/trie4.rs) this does a complete re-tooling of the trie
to change the data layout of the transition table. The transition table no
longer includes the match state information _and_ we now determine whether a
state is matching or not based entirely on its node ID. This does come with
some costs (like a bigger representation in memory) that would probably need to
be mitigated before actually being used, but it does result in a much faster
speed-up by doing less work in the core search loop.
* [`src/trie5.rs`](src/trie5.rs) this final attempt evolved from `trie4` and
took a trick from the scan-and-`match` technique: we require the caller to do
their own scan to find the full label first. This essentially admits defeat on
my original idea to do the matching in a single pass, but I wanted to see if
this would help the trie approach. In particular, by requiring that _only_ the
candidate label be given, the core search loop for the trie can be simplified
to do less work.

To get a sense for how performance across these trie implementations differs,
we can use `critcmp` (after having run the benchmarks, as shown above):

```
$ critcmp friendly -g '.*?/(.*)' -f 'by-trie[1245]'
group     friendly/by-trie1/                     friendly/by-trie2/                     friendly/by-trie4/                     friendly/by-trie5/
-----     ------------------                     ------------------                     ------------------                     ------------------
long      2.67     27.9±0.18ns        ? ?/sec    2.56     26.8±0.24ns        ? ?/sec    1.12     11.8±0.07ns        ? ?/sec    1.00     10.5±0.06ns        ? ?/sec
medium    3.35     16.5±0.11ns        ? ?/sec    3.10     15.3±0.08ns        ? ?/sec    1.18      5.8±0.18ns        ? ?/sec    1.00      4.9±0.10ns        ? ?/sec
short     3.03      6.6±0.05ns        ? ?/sec    3.14      6.8±0.04ns        ? ?/sec    1.25      2.7±0.02ns        ? ?/sec    1.00      2.2±0.04ns        ? ?/sec
```

(I omitted `by-trie3` for space reasons and because its timing isn't
meaningfully different from `by-trie2`.)

Of these, `by-trie5` seems to have the best search performance (although in
some runs, `by-trie4` did better, so there is some noise in the results).
But both `by-trie4` and `by-trie5` are pretty comparable, especially when
contrasted with `by-trie1` and `by-trie2`. The main change accounting for the
performance improvement of `by-trie{4,5}` over `by-trie{1,2,3}` is in how match
states are recognized. In `by-trie{4,5}`, we can tell if a state is matching or
not merely through a comparison involving its state identifier. In contrast,
with `by-trie{1,2,3}`, we need to consult some state in the transition table to
determine if the state is matching or not. It's just more expensive.

> **NOTE:** There are other variations on `by-trie` that could be benchmarked.
> I even tried some of them as intermediate states. For example, getting rid of
> the equivalence class optimization that shrinks the alphabet of the finite
> state machine without sacrificing fast transition look-ups. But all such
> changes I tried didn't result in an improvement. In some cases, they led to
> something slower.

So, using `by-trie5` as our "best overall" implementation, let's compare it
with `one-big-match`:

```
$ critcmp friendly -g '.*?/(.*)' -f 'by-trie5|one-big-match/'
group     friendly/by-trie5/                     friendly/one-big-match/
-----     ------------------                     -----------------------
long      1.66     10.9±0.19ns        ? ?/sec    1.00      6.6±0.25ns        ? ?/sec
medium    1.49      6.2±0.09ns        ? ?/sec    1.00      4.2±0.09ns        ? ?/sec
short     1.00      2.7±0.07ns        ? ?/sec    1.07      2.9±0.05ns        ? ?/sec
```

So `one-big-match` is still a bit faster, but `by-trie5` is pretty close. As
is `by-trie4`:

```
$ critcmp friendly -g '.*?/(.*)' -f 'by-trie4|one-big-match/'
group     friendly/by-trie4/                     friendly/one-big-match/
-----     ------------------                     -----------------------
long      1.53     10.0±0.18ns        ? ?/sec    1.00      6.6±0.25ns        ? ?/sec
medium    1.45      6.1±0.13ns        ? ?/sec    1.00      4.2±0.09ns        ? ?/sec
short     1.00      2.8±0.02ns        ? ?/sec    1.06      2.9±0.05ns        ? ?/sec
```

With that said, I had been thinking that the trie approach here would be much
better. Or perhaps stated differently, I didn't think `one-big-match` would be
as fast as it is here. Moreover, one problem with `by-trie5` is that it uses
about 3 times the amount of memory that `by-trie4` does. I think this could be
mitigated somewhat with some effort, but I'm not sure if it can be as compact
due to the pre-multiplied state identifiers. (And it seems like `by-trie4` is
faster anyway, although this has oscillated back-and-forth as I've run the
benchmarks, so there's clearly some noise here.)

Because of that, my thinking is that the memory usage, complexity and extra
code that come with the trie approach isn't worth it over the `one-big-match`
approach.

### Aho-Corasick

Given that I built the `aho-corasick` crate, my instinct was that it wasn't
going to be as fast as my hand-rolled trie. Why? Because the `aho-corasick`
crate, like the `regex` crate, has a fair bit of overhead involved in every
search to account for everything it supports. As one example, its search
routines won't be inlined into caller code, but the trie's search routine is
simple enough for us to do it. So especially for the shorter labels, I'd expect
even something like my initial trie implementation to beat `aho-corasick` by
having less overhead. However, I wanted to incorporate it in this benchmark as
a signpost. So let's see how it fairs:

```
$ critcmp friendly -g '.*?/(.*)' -f 'by-trie[145]|aho-corasick'
group     friendly/aho-corasick/                 friendly/by-trie1/                     friendly/by-trie4/                     friendly/by-trie5/
-----     ----------------------                 ------------------                     ------------------                     ------------------
long      1.60     16.0±0.10ns        ? ?/sec    2.78     27.9±0.12ns        ? ?/sec    1.00     10.0±0.18ns        ? ?/sec    1.09     10.9±0.19ns        ? ?/sec
medium    1.79     10.9±0.09ns        ? ?/sec    2.53     15.4±0.07ns        ? ?/sec    1.00      6.1±0.13ns        ? ?/sec    1.03      6.2±0.09ns        ? ?/sec
short     2.00      5.5±0.03ns        ? ?/sec    2.42      6.6±0.09ns        ? ?/sec    1.01      2.8±0.02ns        ? ?/sec    1.00      2.7±0.07ns        ? ?/sec
```

Interestingly, `aho-corasick` does beat my initial trie implementation! I
was pleasantly surprised by how little overhead `aho-corasick` has. Alas,
`by-trie4` and `by-trie5` do do a fair bit better than `aho-corasick`,
particularly on the short benchmark.

### Perfect hash functions

Another thought I had for quick parsing of unit designator labels was perfect
hashing. In particular, I was drawn to [Cichelli's method] given how cheap its
(suggested) hash function was. It doesn't even examine every byte in the label.
But its technique doesn't seem to be easily adaptable to the unit designator
labels we want to recognize. That is, I couldn't come up with a scheme that
avoided collisions.

Moreover, I specifically wanted a _minimal_ perfect hash function. That is, a
function that creates a mapping from each of the 56 unit designator labels to
a distinct number in the range `0..56`. It would _probably_ be fine for it not
to be minimal. Like, if we could only manage a mapping into the range `0..100`,
or even `0..500`, then that would totally work too. The point is that the range
is small enough that we can use constant time indexing to map the hash to the
corresponding `Unit`.

The reason why I looked into this method is because we could do this in a
single pass over the needle _and_ it shouldn't require needing to traverse any
transition table. But in my brief review of perfect hash functions, it seems
like most techniques do end up involving consulting some tabular data in one
form or another, or otherwise uses some kind of nesting.

I did try the `phf` crate, but it didn't do as well as `by-trie4`:

```
$ critcmp friendly -g '.*?/(.*)' -f 'by-trie[14]|phf'
group     friendly/by-trie1/                     friendly/by-trie4/                     friendly/phf/
-----     ------------------                     ------------------                     -------------
long      2.78     27.9±0.12ns        ? ?/sec    1.00     10.0±0.18ns        ? ?/sec    1.65     16.6±0.08ns        ? ?/sec
medium    2.53     15.4±0.07ns        ? ?/sec    1.00      6.1±0.13ns        ? ?/sec    2.23     13.6±0.09ns        ? ?/sec
short     2.39      6.6±0.09ns        ? ?/sec    1.00      2.8±0.02ns        ? ?/sec    3.86     10.6±0.07ns        ? ?/sec
```

I do wonder if there is a way to devise a perfect hash function tailored to
the specific 56 unit designator labels supported by Jiff that doesn't require
the overhead of a more general solution. Basically, what Cichelli did for
recognizing Pascal's list of reserved words. I have a suspicion that it could
lead to the fastest possible implementation assuming a very cheap hash
function can be found.

### Generated DFA

One clue I got from the `one-big-match` approach came from looking at its
generated code. The Assembly it creates is clearly a state machine. My trie is
also a state machine, but it isn't represented in the code like `one-big-match`
is (it uses an in-memory transition table instead). So, what if I generated
my own state machine in the code but in such a way that it doesn't require
the two-pass approach of `one-big-match`? Maybe I can get the Assembly code
generated to look like `one-big-match`, but without the initial scan to find
the label.

So I wrote a [Rust code generator for DFAs based on
`regex-automata`](gendfa/main.rs). It's not a fully general generator and is
instead pretty tightly coupled to the kind of DFA we need for recognizing
unit designator labels. But it might be a good example to follow if you need
something similar.

To make things concrete, here is what the generated Rust code looks like for
recognizing just the `yrs`, `mos` and `hrs` labels:

```rust
use crate::Unit;

#[inline(always)]
pub(super) fn find(haystack: &[u8]) -> Option<(Unit, usize)> {
  let mut sid = State::S0;
  for byte in haystack.iter().copied() {
    sid = match sid {
      State::DEAD => return None,
      State::S0 => {
        match byte {
          b'y' => State::S1,
          b'h' => State::S2,
          b'm' => State::S3,
          _ => State::DEAD,
        }
      }
      State::S1 => {
        match byte {
          b'r' => State::S4,
          _ => State::DEAD,
        }
      }
      State::S2 => {
        match byte {
          b'r' => State::S5,
          _ => State::DEAD,
        }
      }
      State::S3 => {
        match byte {
          b'o' => State::S6,
          _ => State::DEAD,
        }
      }
      State::S4 => {
        match byte {
          b's' => State::S7,
          _ => State::DEAD,
        }
      }
      State::S5 => {
        match byte {
          b's' => State::S8,
          _ => State::DEAD,
        }
      }
      State::S6 => {
        match byte {
          b's' => State::S9,
          _ => State::DEAD,
        }
      }
      State::S7 => {
        match byte {
          b'\x00'..=b'\xff' => State::S10,
        }
      }
      State::S8 => {
        match byte {
          b'\x00'..=b'\xff' => State::S11,
        }
      }
      State::S9 => {
        match byte {
          b'\x00'..=b'\xff' => State::S12,
        }
      }
      State::S10 => {
        return Some((Unit::Year, 3));
      }
      State::S11 => {
        return Some((Unit::Hour, 3));
      }
      State::S12 => {
        return Some((Unit::Month, 3));
      }
    };
  }
  return match sid {
    State::S7 => Some((Unit::Year, 3)),
    State::S8 => Some((Unit::Hour, 3)),
    State::S9 => Some((Unit::Month, 3)),
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
  }
}
```

The [code for recognizing all 56 unit designator labels](src/gendfa1.rs) is
quite a bit bigger, but it looks about the same as above. You can generate
the DFA like so:

```
cargo r -rqp gendfa
```

One interesting aspect of Rust is that it doesn't have `goto`, so it's not
quite clear (to me) how to write Rust in a way that leads to optimal codegen.
But let's see how the above fairs:

```
$ critcmp friendly -g '.*?/(.*)' -f 'by-trie5|one-big-match/|gendfa1'
group     friendly/by-gendfa1/                   friendly/by-trie5/                     friendly/one-big-match/
-----     --------------------                   ------------------                     -----------------------
long      1.99     13.0±0.05ns        ? ?/sec    1.66     10.9±0.19ns        ? ?/sec    1.00      6.6±0.25ns        ? ?/sec
medium    1.89      7.9±0.03ns        ? ?/sec    1.49      6.2±0.09ns        ? ?/sec    1.00      4.2±0.09ns        ? ?/sec
short     1.42      3.9±0.05ns        ? ?/sec    1.00      2.7±0.07ns        ? ?/sec    1.07      2.9±0.05ns        ? ?/sec
```

Well... that's disappointing. My `gendfa1` is slower than `one-big-match`
across the board. I was hoping it would be _faster_ since it works in a single
pass. Looking at the codegen for `gendfa1`, it looks like a state machine, but
there are a lot more `mov` instructions than in `one-big-match`. I don't know
enough about optimizing compilers to figure out what I'm doing wrong, or even
if convincing `rustc` to emit better codegen is possible in this case.

### Generated DFA... in C

While Rust lacks `goto`, we do have access to a language that can be very
easily incorporated into a Rust crate at little cost: C. So I modified the
[DFA code generator](gendfa/main.rs) to emit C code in addition to Rust code.
Just pass the `--c` flag.

```
cargo r -rqp gendfa -- --c
```

Here's what the output looks like for the same `yrs`, `mos` and `hrs` example
as above:

```c
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
    case 'y': goto S1;
    case 'h': goto S2;
    case 'm': goto S3;
    default: goto DONE;
  }
S1:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'r': goto S4;
    default: goto DONE;
  }
S2:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'r': goto S5;
    default: goto DONE;
  }
S3:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 'o': goto S6;
    default: goto DONE;
  }
S4:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 's': goto S7;
    default: goto DONE;
  }
S5:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 's': goto S8;
    default: goto DONE;
  }
S6:
  if (p >= end) {
    goto DONE;
  }
  switch (*p++) {
    case 's': goto S9;
    default: goto DONE;
  }
S7:
  o.unit = Year;
  o.length = 3;
  goto DONE;
S8:
  o.unit = Hour;
  o.length = 3;
  goto DONE;
S9:
  o.unit = Month;
  o.length = 3;
  goto DONE;
DONE:
  return o;
}
```

The code here is much tighter than the Rust version. Notably, there's no
looping over bytes and there's no need to `match` over state identifiers. You
just represent each DFA state with a label in the source code and use `goto`
to jump to it. It's very nice.

So how does C compare?

```
$ critcmp friendly -g '.*?/(.*)' -f 'by-trie5|one-big-match/|gendfa1|gencdfa1'
group     friendly/by-gencdfa1/                  friendly/by-gendfa1/                   friendly/by-trie5/                     friendly/one-big-match/
-----     ---------------------                  --------------------                   ------------------                     -----------------------
long      1.00      4.8±0.12ns        ? ?/sec    2.72     13.0±0.05ns        ? ?/sec    2.28     10.9±0.19ns        ? ?/sec    1.37      6.6±0.25ns        ? ?/sec
medium    1.00      3.5±0.11ns        ? ?/sec    2.26      7.9±0.03ns        ? ?/sec    1.79      6.2±0.09ns        ? ?/sec    1.20      4.2±0.09ns        ? ?/sec
short     1.00      2.6±0.05ns        ? ?/sec    1.46      3.9±0.05ns        ? ?/sec    1.03      2.7±0.07ns        ? ?/sec    1.10      2.9±0.05ns        ? ?/sec
```

Wow. Finally. We've handedly beaten the state machine generated by
`one-big-match`. And the C state machine is faster than everything else.

I'd love for Rust to be able to generate code like we did with C above because
I don't want to add a dependency on C to Jiff. Moreover, because of the FFI
boundary, inlining becomes difficult (while not impossible as I understand
it, it's not something I'd expect a typical setup to enable, or at least, I
couldn't get it to work), which presumably incurs a performance penalty here.
So in theory, if we could get Rust code to generate the same thing as the C
code, it might be even faster because it would be more easily inlined.

### One big match... but with prefix matching

Much to my chagrin, I did not think to try this initially. But, you can
actually match on prefixes of slices. Like this:

```rust
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
    // ... and so on
    &[b'y', ..] => Some((Unit::Year, 1)),
    &[b'w', ..] => Some((Unit::Week, 1)),
    &[b's', ..] => Some((Unit::Second, 1)),
    &[b'm', ..] => Some((Unit::Minute, 1)),
    &[b'h', ..] => Some((Unit::Hour, 1)),
    &[b'd', ..] => Some((Unit::Day, 1)),
    _ => None,
}
```

This eliminates our two-pass approach when using `one-big-match` and gives the
compiler a bit more information about what it is we're trying to do. The only
catch is that we need to make sure we've sorted our prefixes with the longest
coming first, otherwise something like `m` (for minutes) will always match,
even when the label is `millis`. But this is fine for this particular set of
words.

Since the above is supremely annoying to write by hand for even
modestly sized sets of words, I wrote a [prefix `match` generator for
it](gen-match-prefix/main.rs).

So how does it compare with the generated DFA in C code?

```
$ critcmp friendly -g '.*?/(.*)' -f 'gencdfa1|one-big-match-prefix'
group     friendly/by-gencdfa1/                  friendly/one-big-match-prefix/
-----     ---------------------                  ------------------------------
long      1.90      4.8±0.12ns        ? ?/sec    1.00      2.5±0.06ns        ? ?/sec
medium    1.77      3.5±0.11ns        ? ?/sec    1.00      2.0±0.06ns        ? ?/sec
short     1.00      2.6±0.05ns        ? ?/sec    1.25      3.3±0.09ns        ? ?/sec
```

Nice. So quite a bit better on `medium` and `long`, and slightly slower on
`short`. I find it _very_ interesting that both `medium` and `long` are
actually faster than `short`. I wonder if the compiler using some kind of jump
table based on length. Namely, the `long` benchmark measures the time it takes
to recognize `milliseconds`, which is 12 bytes long. There is only one other
label of that length: `microseconds`. But I'm not quite sure how it would use
this information to speed things up. Every byte still needs to be checked.

So in the end, we were able to generate better code than we did manually with
C, but I don't know if that would work for every case. If our problem were
more complicated, like, say, encoding an arbitrary state machine that couldn't
be expressed as a `match` statement of literal prefixes, then C might have an
edge here that is difficult or impossible to beat with Rust.

## Questions

I don't think my above exploration is exhaustive. And there are definitely some
unanswered questions that I haven't really dug into fully yet. I generally
didn't analyze the code generated too closely to get a better sense of what
precisely each technique was actually doing.

Firstly, can we come up with a minimal (or nearly minimal) perfect hash
function tailored to these unit designator labels that is cache efficient?

Secondly, can we generate a DFA in Rust source code that matches the search
performance of the C version? Rust doesn't have `goto`, but optimizing compilers
are impressive beasts. Is there a way we can nudge it in the direction of
code produced by a C compiler for this specific case?

Thirdly, I'd like to better understand what exactly makes `gendfa1` (the Rust
generated DFA) so much slower than both `gencdfa1` and `one-big-match`. The
generated Assembly _looks_ like a state machine, so it seems like we're _close_
to doing the right thing. But it's just not quite there.

Fourthly, I didn't really explore the use of SIMD for this task. The
`aho-corasick` crate does have a SIMD multi-pattern matcher (called Teddy),
but because some of the labels are very short (one byte), my guess is that its
fingerprinting approach won't work as well. But I should still add it to these
benchmarks.

[`aho-corasick`]: https://docs.rs/aho-corasick
[`critcmp`]: https://github.com/BurntSushi/critcmp
[Cichelli's method]: https://courses.cs.vt.edu/cs3114/Spring18/wmcquain/Notes/Supplemental/p17-cichelli.pdf
[`regex-automata`]: https://docs.rs/regex-automata
