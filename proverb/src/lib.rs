pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb: Vec<String> = list
        .iter()
        .enumerate()
        .map(|(i, elem)| {
            if let Some(next) = list.get(i + 1) {
                let mut verse = String::new();
                verse.push_str(&format!("For want of a {} ", elem));
                verse.push_str(&format!("the {} was lost.", next));
                Some(verse)
            } else {
                None
            }
        })
        .flatten()
        .collect();
    if let Some(elem) = list.first() {
        proverb.push(format!("And all for the want of a {}.", elem));
    }
    proverb.join("\n")
}
