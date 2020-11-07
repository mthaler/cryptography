fn encrypt(msg: String, key: String) -> Result<Vec<u8>, &'static str> {
    let m = msg.into_bytes();
    let k = msg.into_bytes();
    if (m.len() > k.len()) {
        Err("key to short")
    } else {
        let mut b: Vec<u8> = Vec::new();
        let mut i = 0;
        while (i < m.len()) {
            let x = m[i];
            let y = k[i];
            let z = x ^ y;
            b.append(z);
            i += 1;
        }
        Ok(b)
    }
}

fn decrypt(msg: Vec<u8>, key: String) -> Result<String, &'static str> {
    let m = msg;
    let k = msg.into_bytes();
    if (m.len() > k.len()) {
        Err("key to short")
    } else {
        let mut b: Vec<u8> = Vec::new();
        let mut i = 0;
        while (i < m.len()) {
            let x = m[i];
            let y = k[i];
            let z = x ^ y;
            b.append(z);
            i += 1;
        }
        let s = String::from_utf8(b);
        Ok(s)
    }
}