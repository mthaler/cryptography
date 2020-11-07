use rand::Rng;

pub fn encrypt(msg: String, key: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
    let m = msg.into_bytes();
    if m.len() > key.len() {
        Err("key to short")
    } else {
        let mut b: Vec<u8> = Vec::new();
        let mut i = 0;
        while i < m.len() {
            let x = m[i];
            let y = key[i];
            let z = x ^ y;
            b.push(z);
            i += 1;
        }
        Ok(b)
    }
}

pub fn decrypt(msg: Vec<u8>, key: &Vec<u8>) -> Result<String, &'static str> {
    if msg.len() > key.len() {
        Err("key to short")
    } else {
        let mut b: Vec<u8> = Vec::new();
        let mut i = 0;
        while i < msg.len() {
            let x = msg[i];
            let y = key[i];
            let z = x ^ y;
            b.push(z);
            i += 1;
        }
        let s = String::from_utf8(b).expect("Could not convert to ");
        Ok(s)
    }
}


fn generate_key(n: i32) -> Vec<u8> {
    let mut key: Vec<u8> = Vec::new();
    let mut i = 0;
    let mut rng = rand::thread_rng();
    while i < n {
        let x = rng.gen::<u8>();
        key.push(x);
        i += 1;
    }
    key
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn encrypt_decrypt() {
        use super::{encrypt, decrypt, generate_key};
        
        let msg =  "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, \n
it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair, we had everything before us, we had nothing before us, \n
we were all going direct to Heaven, we were all going direct the other way--in short, the period was so far like the present period, that some of its noisiest authorities insisted \n
on its being received, for good or for evil, in the superlative degree of comparison only.";
        let key = generate_key(1024);
        let enc = encrypt(String::from(msg), &key).expect("Could not encrypt msg!");
        let dec = decrypt(enc, &key).expect("Could not decrypt msg!");
        assert_eq!(msg, dec);
    }
}