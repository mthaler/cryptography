use std::collections::HashMap;

fn generate_key(n: i32) -> HashMap<char, char> {
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    let mut key: HashMap<char, char> = HashMap::new();
    let mut i = 0;
    while i < letters.len() {
        key.insert(letters[i], letters[((i + n as usize) % letters.len())]);
        i += 1;
    }
    key
}

fn encrypt(key: HashMap<char, char>, msg: String) -> String {
    let mut s = String::new();
    for c in msg.chars() {
        match key.get(&c) {
            Some(x) =>
                s.push(*x),
            None =>
                s.push(c),
        }
    }
    s
}

fn decrypt(key: HashMap<char, char>, msg: String) -> String {
    let mut inverted: HashMap<char, char> = HashMap::new();
    for (k, v) in key {
        inverted.insert(v, k);
    }
    let mut s = String::new();
    for c in msg.chars() {
        match inverted.get(&c) {
            Some(x) =>
                s.push(*x),
            None =>
                s.push(c),
        }
    }
    s
}

#[cfg(test)]
mod tests {

    const MSG: &str =  "It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, \n
it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair, we had everything before us, we had nothing before us, \n
we were all going direct to Heaven, we were all going direct the other way--in short, the period was so far like the present period, that some of its noisiest authorities insisted \n
on its being received, for good or for evil, in the superlative degree of comparison only.";

    const CYPHER: &str = "Lw zdv wkh ehvw ri wlphv, lw zdv wkh zruvw ri wlphv, lw zdv wkh djh ri zlvgrp, lw zdv wkh djh ri irrolvkqhvv, lw zdv wkh hsrfk ri eholhi, lw zdv wkh hsrfk ri lqfuhgxolwB, \n
lw zdv wkh vhdvrq ri Oljkw, lw zdv wkh vhdvrq ri Gdunqhvv, lw zdv wkh vsulqj ri krsh, lw zdv wkh zlqwhu ri ghvsdlu, zh kdg hyhuBwklqj ehiruh xv, zh kdg qrwklqj ehiruh xv, \n
zh zhuh doo jrlqj gluhfw wr Khdyhq, zh zhuh doo jrlqj gluhfw wkh rwkhu zdB--lq vkruw, wkh shulrg zdv vr idu olnh wkh suhvhqw shulrg, wkdw vrph ri lwv qrlvlhvw dxwkrulwlhv lqvlvwhg \n
rq lwv ehlqj uhfhlyhg, iru jrrg ru iru hylo, lq wkh vxshuodwlyh ghjuhh ri frpsdulvrq rqoB.";

    #[test]
    fn test_generate_key() {
        let key = super::generate_key(3);
        assert_eq!(key.get(&'A'), Some(&'D'))
    }

    #[test]
    fn test_encrypt() {
        let key = super::generate_key(3);
        let result = super::encrypt(key, String::from(MSG));
        assert_eq!(result, CYPHER);
    }

    #[test]
    fn test_decrypt() {
        let key = super::generate_key(3);
        let result = super::decrypt(key, String::from(CYPHER));
        assert_eq!(result, MSG);
    }
}