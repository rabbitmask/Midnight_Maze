pub(crate) fn xor_encrypt(shellcode: &[u8], key: &[u8]) -> String {
    let mut encrypted = Vec::new();
    for (i, &b) in shellcode.iter().enumerate() {
        encrypted.push(b ^ key[i % key.len()]);
    }
    hex::encode(&encrypted)
}

pub(crate) fn xor_decrypt(encrypted: &[u8], key: &[u8]) -> Vec<u8> {
    let encrypted = hex::decode(encrypted).expect("Error");

    let mut decrypted = Vec::new();
    for (i, &b) in encrypted.iter().enumerate() {
        decrypted.push(b ^ key[i % key.len()]);
    }
    decrypted
}

pub(crate) fn add_random(xor_string: &str, key: &str) -> String {
    let mut result = String::new();

    for (i, c) in xor_string.chars().enumerate() {
        result.push(c);
        result.push(key.chars().nth(i % key.len()).unwrap());
    }
    hex::encode(&result)
}

pub(crate) fn rm_random(random_string: &str) -> Vec<u8> {
    let mut result = String::new();

    let random_string = hex::decode(random_string).expect("Invalid String");
    let random_string = match std::str::from_utf8(random_string.as_slice()) {
        Ok(s) => s,
        Err(_) => "Invalid UTF-8 sequence",
    };

    for (i, c) in random_string.chars().enumerate() {
        if i % 2 == 0 {
            result.push(c);
        }
    }
    result.as_bytes().to_vec()
}


pub(crate) fn xor_random_demo(){
    let shellcode0 = &[0xfc,0x48];
    println!("\nshellcode0:{:?}", shellcode0);
    let key = "GjGKKyYUF778V5Vhvvhjhsd798GCKnx";
    println!("Key: {}", key);

    let encrypted0 =xor_encrypt(shellcode0,key.as_bytes());
    println!("\n{}", encrypted0);

    let random_string = add_random(encrypted0.as_str(), key);
    println!("\n{}", random_string);


    let encrypted = rm_random(random_string.as_str());
    println!("\n{:?}", encrypted);

    let shellcode = xor_decrypt(encrypted.as_slice(),key.as_bytes());
    println!("\nshellcode:{:?}", shellcode);
}
