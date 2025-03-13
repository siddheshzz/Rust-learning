use std::collections::HashSet;

fn ordered_word(word: &str) -> Vec<char>{
    let mut chars:Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}

pub fn anagram_for(word: &str, candidates: &[&str]) -> HashSet<String> {
    let word = ordered_word(word);
    candidates
        .iter()
        .filter(|x| ordered_word(x) == word)
        .map(|x| x.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram() {
        let word = "orchestra";
        let inputs = &["cashregister", "Carthorse", "radishes"];
        let output = anagram_for(word, inputs);
        let expected = HashSet::from(["Carthorse".to_string()]);

        assert_eq!(output, expected);
    }
}
