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
    let candidates: Vec<(usize, String)> = reader
        .lines()
        .enumerate()
        .filter_map(move |(n, line)| match line {
            Ok(ref line) if contains(pat, &line, flags) => Some((n, line.clone())),
            _ => None,
        })
        .collect();
    if flags.print_name_of_files && candidates.len() > 0 {
        return Ok(vec![path.to_string()]);
    }
    let show_fn = if mult {
        format!("{}:", path)
    } else {
        "".to_string()
    };
    Ok(candidates
        .iter()
        .map(|(n, line)| {
            let show_ln = if flags.print_line_numbers {
                format!("{}:", n + 1)
            } else {
                "".to_string()
            };
            format!("{}{}{}", show_fn, show_ln, line)
        })
        .collect())
}

fn contains(pattern: &str, line: &str, flags: &Flags) -> bool {
    let pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };
    let line = if flags.case_insensitive {
        line.to_lowercase()
    } else {
        line.to_string()
    };

    let result = if flags.match_entire_lines {
        pattern == line
    } else {
        line.contains(&pattern)
    };
    if flags.invert_search {
        !result
    } else {
        result
    }
}