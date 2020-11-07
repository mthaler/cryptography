use rand::Rng;
use std::collections::HashMap;

fn generate_key() -> HashMap<char, char> {
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut cletters = letters.chars().collect::<Vec<char>>();
    let mut rng = rand::thread_rng();
    let mut key: HashMap<char, char> = HashMap::new();
    for c in letters.chars() {
        let pos = rng.gen_range(0, cletters.len());
        let to = cletters[pos];
        cletters.remove(pos);
        key.insert(c, to);
    }
    key
}

fn encrypt(key: &HashMap<char, char>, msg: String) -> String {
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

fn decrypt(key: &HashMap<char, char>, msg: String) -> String {
    let mut inverted: HashMap<char, char> = HashMap::new();
    for (k, v) in key {
        inverted.insert(*v, *k);
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

    #[test]
    fn test_generate_key() {
        let key = super::generate_key();
        assert_eq!(key.len(), 52)
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = super::generate_key();
        let cypher = super::encrypt(&key, String::from(MSG));
        let result = super::decrypt(&key, cypher);
        assert_eq!(result, MSG)
    }
}