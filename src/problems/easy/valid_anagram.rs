pub fn valid_anagram(s: String, t: String) -> bool {
    let mut counts = [0; 26];

    for (c1, c2) in s.chars().zip(t.chars()) {
        let idx1 = (c1 as u8 - b'a') as usize;
        let idx2 = (c2 as u8 - b'a') as usize;
        counts[idx1] += 1;
        counts[idx2] -= 1;
    }

    for c in counts {
        if c != 0 {
            return false;
        }
    }

    true
}
