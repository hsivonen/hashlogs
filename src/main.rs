// Copyright 2018 Henri Sivonen. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rand;
extern crate blake2_rfc;
extern crate data_encoding;

use std::io::BufRead;
use std::io::Write;
use rand::Rng;

#[inline(always)]
pub fn first_space(buffer: &[u8]) -> Option<usize> {
	buffer.iter().position(|&x| x == b' ')
}

fn main() {
	let mut key = [0u8; 16];
	let mut rng = rand::os::OsRng::new().expect("Failed to open system random number generator");
	rng.fill_bytes(&mut key[..]);

	let mut hex = [0u8; 16];

	let stdout = std::io::stdout();
	let mut out_handle = stdout.lock();

	let stdin = std::io::stdin();
	let mut handle = stdin.lock();
	let mut line: Vec<u8> = Vec::new();
	loop {
		line.truncate(0);
		let _ = handle.read_until(b'\n', &mut line).expect("IO error on input");
		if line.is_empty() {
			break;
		}
		if let Some(space_pos) = first_space(&line) {
			let (addr, tail) = line.split_at(space_pos);
			let hash = blake2_rfc::blake2b::blake2b(8, &key[..], addr);
			data_encoding::HEXUPPER.encode_mut(hash.as_bytes(), &mut hex[..]);
			out_handle.write_all(&hex[..]).expect("IO error on output");
			out_handle.write_all(tail).expect("IO error on output");
		} // else skip bad line
	}
	out_handle.flush().expect("IO error on output");
}
