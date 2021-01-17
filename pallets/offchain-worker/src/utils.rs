//! Based on https://github.com/debris/base58/blob/master/src/lib.rs
//! works only up to 128 bytes
use sp_std::prelude::*;

// const ALPHABET: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// /// A trait for converting a value to base58 encoded string.
// pub trait ToBase58 {
// 	/// Converts a value of `self` to a base58 value, returning the owned string.
// 	fn to_base58(&self) -> Vec<u8>;
// }

// impl ToBase58 for [u8] {
// 	fn to_base58(&self) -> Vec<u8> {
// 		let zcount = self.iter().take_while(|x| **x == 0).count();
// 		let size = (self.len() - zcount) * 138 / 100 + 1;
// 		let mut buffer = vec![0u8; size];

// 		let mut i = zcount;
// 		let mut high = size - 1;

// 		while i < self.len() {
// 			let mut carry = self[i] as u32;
// 			let mut j = size - 1;

// 			while j > high || carry != 0 {
// 				carry += 256 * buffer[j] as u32;
// 				buffer[j] = (carry % 58) as u8;
// 				carry /= 58;

// 				// in original trezor implementation it was underflowing
// 				if j  > 0 {
// 					j -= 1;
// 				}
// 			}

// 			i += 1;
// 			high = j;
// 		}

// 		let mut j = buffer.iter().take_while(|x| **x == 0).count();

// 		let mut result = Vec::new();
// 		for _ in 0..zcount {
// 			result.push(b'1');
// 		}

// 		while j < size {
// 			result.push(ALPHABET[buffer[j] as usize]);
// 			j += 1;
// 		}

// 		result
// 	}
// }

// u128 number string to u128
pub fn chars_to_u128(vec: &Vec<char>) -> Result<u128, &'static str> {
	// Check if the number string is decimal or hexadecimal (whether starting with 0x or not) 
	let base = if vec.len() >= 2 && vec[0] == '0' && vec[1] == 'x' {
		// This is a hexadecimal number
		16
	} else {
		// This is a decimal number
		10
	};

	let mut result: u128 = 0;
	for (i, item) in vec.iter().enumerate() {
		// Skip the 0 and x digit for hex. 
		// Using skip here instead of a new vec build to avoid an unnecessary copy operation
		if base == 16 && i < 2 {
			continue;
		}

		let n = item.to_digit(base);
		match n {
			Some(i) => {
				let i_64 = i as u128; 
				result = result * base as u128 + i_64;
				if result < i_64 {
					return Err("Wrong u128 balance data format");
				}
			},
			None => return Err("Wrong u128 balance data format"),
		}
	}
	return Ok(result)
}

// number byte to string byte
pub fn u8_to_str_byte(a: u8) -> u8{
	if a < 10 {
		return a + 48 as u8;
	}
	else {
		return a + 87 as u8;
	}
}

// address to string bytes
pub fn address_to_string(address: &[u8; 20]) -> Vec<u8> {

	let mut vec_result: Vec<u8> = Vec::new();
	for item in address {
		let a: u8 = item & 0x0F;
		let b: u8 = item >> 4;
		vec_result.push(u8_to_str_byte(b));
		vec_result.push(u8_to_str_byte(a));
	}
	return vec_result;
}

// #[cfg(test)]
// mod tests {
// 	use super::ToBase58;
// 	use std::str::from_utf8;
// 	use hex::decode;

// 	#[test]
// 	fn test_to_base58_basic() {
// 		assert_eq!(from_utf8(&b"".to_base58()).unwrap(), "");
// 		assert_eq!(from_utf8(&[32].to_base58()).unwrap(), "Z");
// 		assert_eq!(from_utf8(&[45].to_base58()).unwrap(), "n");
// 		assert_eq!(from_utf8(&[48].to_base58()).unwrap(), "q");
// 		assert_eq!(from_utf8(&[49].to_base58()).unwrap(), "r");
// 		assert_eq!(from_utf8(&[57].to_base58()).unwrap(), "z");
// 		assert_eq!(from_utf8(&[45, 49].to_base58()).unwrap(), "4SU");
// 		assert_eq!(from_utf8(&[49, 49].to_base58()).unwrap(), "4k8");
// 		assert_eq!(from_utf8(&b"abc".to_base58()).unwrap(), "ZiCa");
// 		assert_eq!(from_utf8(&b"1234598760".to_base58()).unwrap(), "3mJr7AoUXx2Wqd");
// 		assert_eq!(from_utf8(&b"abcdefghijklmnopqrstuvwxyz".to_base58()).unwrap(), "3yxU3u1igY8WkgtjK92fbJQCd4BZiiT1v25f");
// 	}

// 	#[test]
// 	fn test_to_base58_initial_zeros() {
// 		assert_eq!(from_utf8(&b"\0abc".to_base58()).unwrap(), "1ZiCa");
// 		assert_eq!(from_utf8(&b"\0\0abc".to_base58()).unwrap(), "11ZiCa");
// 		assert_eq!(from_utf8(&b"\0\0\0abc".to_base58()).unwrap(), "111ZiCa");
// 		assert_eq!(from_utf8(&b"\0\0\0\0abc".to_base58()).unwrap(), "1111ZiCa");
// 	}

// 	/// https://github.com/bitcoin/bitcoin/blob/master/src/test/data/base58_encode_decode.json
// 	/// NB: left is hex data
// 	#[test]
// 	fn test_to_base58_bitcoin_repo_cases() {
// 		let test_cases = vec![
// 			("", ""),
// 			("61", "2g"),
// 			("626262", "a3gV"),
// 			("636363", "aPEr"),
// 			("73696d706c792061206c6f6e6720737472696e67", "2cFupjhnEsSn59qHXstmK2ffpLv2"),
// 			("00eb15231dfceb60925886b67d065299925915aeb172c06647", "1NS17iag9jJgTHD1VXjvLCEnZuQ3rJDE9L"),
// 			("516b6fcd0f", "ABnLTmg"),
// 			("bf4f89001e670274dd", "3SEo3LWLoPntC"),
// 			("572e4794", "3EFU7m"),
// 			("ecac89cad93923c02321", "EJDM8drfXA6uyA"),
// 			("10c8511e", "Rt5zm"),
// 			("00000000000000000000", "1111111111"),
// 			("000111d38e5fc9071ffcd20b4a763cc9ae4f252bb4e48fd66a835e252ada93ff480d6dd43dc62a641155a5", "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"),
// 			("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff", "1cWB5HCBdLjAuqGGReWE3R3CguuwSjw6RHn39s2yuDRTS5NsBgNiFpWgAnEx6VQi8csexkgYw3mdYrMHr8x9i7aEwP8kZ7vccXWqKDvGv3u1GxFKPuAkn8JCPPGDMf3vMMnbzm6Nh9zh1gcNsMvH3ZNLmP5fSG6DGbbi2tuwMWPthr4boWwCxf7ewSgNQeacyozhKDDQQ1qL5fQFUW52QKUZDZ5fw3KXNQJMcNTcaB723LchjeKun7MuGW5qyCBZYzA1KjofN1gYBV3NqyhQJ3Ns746GNuf9N2pQPmHz4xpnSrrfCvy6TVVz5d4PdrjeshsWQwpZsZGzvbdAdN8MKV5QsBDY")
// 		];

// 		for test_case in test_cases.into_iter() {
// 			let (input, output) = test_case;
// 			let input = decode(input).unwrap();
// 			assert_eq!(from_utf8(&input.to_base58()).unwrap(), output);
// 		}
// 	}
// }
