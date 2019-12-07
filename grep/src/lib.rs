use failure::Error;

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
        let mut print_line_numbers: bool = false;
        let mut print_name_of_files: bool = false;
        let mut case_insensitive: bool = false;
        let mut invert_search: bool = false;
        let mut match_entire_lines: bool = false;
        for &flag in flags.iter() {
            match flag {
                "-n" => print_line_numbers = true,
                "-l" => print_name_of_files = true,
                "-i" => case_insensitive = true,
                "-v" => invert_search = true,
                "-x" => match_entire_lines = true,
                _ => panic!("invalid flag!{}", flag),
            }
        }
        Flags {
            print_line_numbers,
            print_name_of_files,
            case_insensitive,
            invert_search,
            match_entire_lines,
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut result = vec![];
    let multiples = files.len() > 1;
    for file in files.iter() {
        let mut file_result = grep_in_file(pattern, flags, file, multiples)?;
        result.append(&mut file_result);
    }
    Ok(result)
}

fn grep_in_file(
    pattern: &str,
    flags: &Flags,
    file_name: &str,
    multiples: bool,
) -> Result<Vec<String>, Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut result = vec![];
    for (n, line) in reader.lines().enumerate() {
        let line = line?;
        if contains(pattern, &line, flags) {
            let mut output: String = String::new();
            if flags.print_name_of_files {
                result.push(file_name.to_string());
                break;
            }
            if multiples {
                output += &format!("{}:", file_name.to_string());
            }
            if flags.print_line_numbers {
                output += &format!("{}:", n + 1);
            }
            output += &line;
            result.push(output);
        }
    }
    Ok(result)
}

fn contains(pattern: &str, line: &str, flags: &Flags) -> bool {
    let mut pattern = pattern.to_string();
    let mut line = line.to_string();
    if flags.case_insensitive {
        pattern = pattern.to_lowercase();
        line = line.to_lowercase();
    }

    if flags.invert_search {
        if flags.match_entire_lines {
            pattern != line
        } else {
            !line.contains(&pattern)
        }
    } else if flags.match_entire_lines {
        pattern == line
    } else {
        line.contains(&pattern)
    }
}
