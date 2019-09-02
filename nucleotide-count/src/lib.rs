use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => match dna
            .chars()
            .find(|c| *c != 'A' && *c != 'C' && *c != 'G' && *c != 'T')
        {
            Some(c) => Err(c),
            None => Ok(dna.chars().filter(|c| nucleotide == *c).count()),
        },
        c => Err(c),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let nucleotides = ['A', 'C', 'G', 'T'];
    let mut result: HashMap<char, usize> = HashMap::new();
    for c in nucleotides.iter() {
        match count(*c, dna) {
            Err(c) => return Err(c),
            Ok(n) => result.insert(*c, n),
        };
    }
    Ok(result)
}
