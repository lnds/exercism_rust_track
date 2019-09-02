use std::collections::HashMap;

const NUCLEOTIDES: &str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(nucleotide) {
        Err(nucleotide)
    } else {
        match dna.chars().find(|c| !NUCLEOTIDES.contains(*c)) {
            Some(c) => Err(c),
            _ => Ok(dna.matches(nucleotide).count()),
        }
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut result = HashMap::new();
    for c in NUCLEOTIDES.chars() {
        let r = count(c, dna)?;
        result.insert(c, r);
    }
    Ok(result)
}
