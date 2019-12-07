use failure::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Flags {
    print_line_numbers: bool,
    print_name_of_files: bool,
    case_insensitive: bool,
    invert_search: bool,
    match_entire_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            print_line_numbers: flags.contains(&"-n"),
            print_name_of_files: flags.contains(&"-l"),
            case_insensitive: flags.contains(&"-i"),
            invert_search: flags.contains(&"-v"),
            match_entire_lines: flags.contains(&"-x"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let result = files
        .iter()
        .map(|file| grep_in_file(pattern, flags, file, files.len() > 1))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(result.into_iter().flatten().collect())
}

fn grep_in_file(pat: &str, flags: &Flags, path: &str, mult: bool) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(File::open(path)?);
    let show_fn = alt_text(mult, format!("{}:", path));
    let candidates: Vec<String> = reader
        .lines()
        .enumerate()
        .filter_map(move |(n, line)| match line {
            Ok(ref line) if contains(pat, &line, flags) => Some((n, line.clone())),
            _ => None,
        })
        .map(|(n, line)| {
            let show_ln = alt_text(flags.print_line_numbers, format!("{}:", n + 1));
            format!("{}{}{}", show_fn, show_ln, line)
        })
        .collect();
    if flags.print_name_of_files && !candidates.is_empty() {
        return Ok(vec![path.to_string()]);
    }
    Ok(candidates)
}

fn contains(pattern: &str, line: &str, flags: &Flags) -> bool {
    let pattern = lower(&flags, pattern);
    let line = lower(&flags, line);
    flip(&flags, pattern_match(&flags, &pattern, &line))
}

fn pattern_match(flags: &Flags, pattern: &str, line: &str) -> bool {
    if flags.match_entire_lines {
        pattern == line
    } else {
        line.contains(pattern)
    }   
}

fn flip(flags: &Flags, value: bool) -> bool {
    if flags.invert_search {
        !value
    } else {
        value
    }
}

fn lower(flags: &Flags, text: &str) -> String {
    if flags.case_insensitive {
        text.to_lowercase()
    } else {
        text.to_string()
    }
}

fn alt_text(flag: bool, text: String) -> String {
    if flag {
        text
    } else {
        "".to_string()
    }
}
