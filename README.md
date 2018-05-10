# hashlogs

[![crates.io](https://meritbadge.herokuapp.com/hashlogs)](https://crates.io/crates/hashlogs)
[![docs.rs](https://docs.rs/hashlogs/badge.svg)](https://docs.rs/hashlogs/)
[![Apache 2 / MIT dual-licensed](https://img.shields.io/badge/license-Apache%202%20%2F%20MIT-blue.svg)](https://github.com/hsivonen/hashlogs/blob/master/COPYRIGHT)

`hashlogs` is a command-line utility that hashes the part before a space on
each line from stdin with blake2b keyed with an ephemeral randomly-generated
key and writes to stdout.

## Installation

```sh
cargo install hashlogs
```

## Usage

No command line argument. Reads from stdin and writes to stdout.

## Details

When the program starts, it obtains 16 randomly-generated bytes (128 bits) of
entropy from the system random number generator. These bytes are used as the
key to the blake2b hash function. The key is forgotten when the program exits.

This has the effect that during a given run of the program, occurrences of
the same hash input result in the same hash output. However, after the
execution has ended, it's infeasible to try to reverse the hash by brute
force, because the key is not known.

If the hash function was not keyed (or the input wasn't alternatively salted),
it would be feasible to brute force a relatively small input space, such as
the IPv4 address space.

8 bytes of output are taken for each hash. The value is represented as
upper-case hexadecimal (16 ASCII characters).

Lines are considered to be separated by a line feed. Lines that don't contain
a space are _discarded_. The part of each line that precedes the first space
on the line is hashed. The rest of the line is not modified.

Operations are performed on bytes, so character encoding issues are garbage
in, garbage out.

## License

`hashlogs` is licensed under either of

 * [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0), ([LICENSE-APACHE](LICENSE-APACHE))
 * [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Any contribution submitted for inclusion in `hashlogs` by you,
as defined in the Apache License 2.0, shall be dual licensed as above,
without any additional terms or conditions. Not adding a copyright notice as
part of your Contribution is taken as a waiver of copyright notice.