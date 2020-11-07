use std::collections::HashMap;

fn frequencies(s: String) -> HashMap<char, f64> {
    let mut counts: HashMap<char, i32> = HashMap::new();
    let s1 = s.to_lowercase();
    for c in s1.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
	let length = s1.chars().count() as f64;
	let mut frequencies: HashMap<char, f64> = HashMap::new();
	for (c, count) in counts {
        frequencies.insert(c, count as f64 / length);
	}
	frequencies
}