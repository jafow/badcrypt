extern crate hex;

use std::io::{self, Read, Write};


fn xor_repeating_key(plaintext: &str, key: &str) -> Result<Vec<u8>, ()> {
    let b = plaintext.as_bytes();
    let k_iter = key.as_bytes().iter().cycle();

    let zipper = b.iter().zip(k_iter);
    let mut res: Vec<u8> = Vec::new();

    for pair in zipper {
        let (lhs, rhs) = pair;
        let _xor: u8 = lhs ^ rhs;
        res.push(_xor);
    }
    Ok(res)
}

// test final output matches
// https://cryptopals.com/sets/1/challenges/5
#[test]
fn test_repeating_key() {
    let b_raw = xor_repeating_key("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE");

    assert_eq!(
        String::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"),
        hex::encode(b_raw.unwrap())
    )
}


/// echo "my secret plaintext message" | cargo run
///     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
///          Running `/home/verycomputer/secret/codes/target/debug/package-name`
///          3938633c363733263b733924303c32332449
fn main() -> Result<(), ()> {
    // secret key
    let very_secret_key = "TACOS";

    // read stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let raw = xor_repeating_key(&input, very_secret_key).expect("should xor input with the key");
    let hx = hex::encode(raw);

    writeln!(io::stdout(), "{}", hx);
    Ok(())
}
