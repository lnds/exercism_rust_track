use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.data.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.chars().collect::<Vec<_>>().chunks(3).map(|chunk|{
            let key : &str = &chunk.iter().collect::<String>();
            self.name_for(key)
        }).take_while(|p|*p!=Some("stop codon")).collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        data: pairs.iter().copied().collect::<HashMap<_, _>>(),
    }
}
