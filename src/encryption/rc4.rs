use crypto::rc4::Rc4;
use crypto::symmetriccipher::SynchronousStreamCipher;
use std::iter::repeat;
use core::str;

pub(crate) fn rc4_enc(plain: &[u8], key: &str) -> String {
    let mut rc4 = Rc4::new(key.as_bytes());

    let mut result: Vec<u8> = repeat(0).take(plain.len()).collect();

    rc4.process(plain, &mut result);

    base64::encode(&mut result)
}

pub(crate) fn rc4_dec(bs4: &String, key: &str) -> String {
    let mut result = match base64::decode(bs4) {
        Ok(result) => result,
        _ => "".as_bytes().to_vec(),
    };

    let mut rc4 = Rc4::new(key.as_bytes());

    let mut outplain: Vec<u8> = repeat(0).take(result.len()).collect();

    rc4.process(&mut result[..], &mut outplain);

    hex::encode(&mut outplain)
}

pub(crate) fn rc4_demo(){
    let shellcode = &[0xfc,0x48];
    let key = "pRNtb343heAlnPFw5QiPHKxz3Z1dzLsqhiUyBNtTiI21DjUsZ0";

    let bs4_string = rc4_enc(shellcode, key);
    let hex_string = rc4_dec(&bs4_string, key);

    println!("== RC4 ==");
    println!("Key: {}", key);
    println!("Encrypted (Base-64): {}", bs4_string);
    println!("Decrypted (hex): {}", hex_string);
}
