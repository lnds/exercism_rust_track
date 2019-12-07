use failure::Error;
use std::fs;

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
    let r = files
        .iter()
        .map(|file| {
            fs::read_to_string(file).map(|text| {
                let matches = text
                    .lines()
                    .enumerate()
                    .filter(|(_, line)| {
                        if flags.invert_search {
                            !contains(pattern, line, &flags)
                        } else {
                            contains(pattern, line, &flags)
                        }
                    })
                    .map(|(n, line)| format_output(&line, n + 1, &flags, file, files.len() > 1))
                    .collect::<Vec<_>>();
                if flags.print_name_of_files && !matches.is_empty() {
                    vec![file.to_string()]
                } else {
                    matches
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(r.into_iter().flatten().collect::<Vec<String>>())
}

fn contains(pattern: &str, line: &str, flags: &Flags) -> bool {
    let pattern = lower(&flags, pattern);
    let line = lower(&flags, line);
    if flags.match_entire_lines {
        pattern == line
    } else {
        line.contains(&pattern)
    }
}

fn lower(flags: &Flags, text: &str) -> String {
    if flags.case_insensitive {
        text.to_lowercase()
    } else {
        text.to_string()
    }
}

fn format_output(line: &str, n: usize, flags: &Flags, fname: &str, multi: bool) -> String {
    format!(
        "{}{}{}",
        if multi {
            format!("{}:", fname)
        } else {
            "".to_string()
        },
        if flags.print_line_numbers {
            format!("{}:", n)
        } else {
            "".to_string()
        },
        line
    )
}
