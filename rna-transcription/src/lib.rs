use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct DNA {
    string: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    string: Vec<char>,
}

const VALID_DNA_NUCLEOTID: &str = "GCTA";

const VALID_RNA_NUCLEOTID: &str = "CGAU";

fn construct_string(seq: &str, valid: &str) -> Result<Vec<char>, usize> {
    let mut result = vec![];
    for (i, c) in seq.chars().enumerate() {
        if !valid.contains(c) {
            return Err(i);
        } else {
            result.push(c)
        }
    }
    Ok(result)
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let string = construct_string(dna, VALID_DNA_NUCLEOTID)?;
        Ok(DNA { string })
    }

    pub fn into_rna(self) -> RNA {
        let map: HashMap<char, char> = VALID_DNA_NUCLEOTID
            .chars()
            .zip(VALID_RNA_NUCLEOTID.chars())
            .collect();
        let string: Vec<char> = self.string.iter().map(|c| map[c]).collect();
        RNA { string }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let string = construct_string(rna, VALID_RNA_NUCLEOTID)?;
        Ok(RNA { string })
    }
}
