// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines = input.lines().collect::<Vec<_>>();
    let rows = lines.len();
    if rows % 4 != 0 {
        return Err(Error::InvalidRowCount(4));
    }
    let cols: usize = lines.iter().map(|l| l.len()).sum();
    if cols % 3 != 0 {
        return Err(Error::InvalidColumnCount(cols / rows));
    }
    Ok(parse_rows(&lines))
}

///  _       _  _       _   _   _   _   _   _
/// |_|   |  _| _| |_| |_  |_    | |_| |_| | |
/// |_|   | |_  _|   |  _| |_|   | |_|  _| |_|
///
fn parse_num(code: &str) -> String {
    match code {
        " _ | ||_|   " => "0",
        "     |  |   " => "1",
        "   |_|  |   " => "4",
        " _  _||_    " => "2",
        " _  _| _|   " => "3",
        " _ |_  _|   " => "5",
        " _ |_ |_|   " => "6",
        " _   |  |   " => "7",
        " _ |_||_|   " => "8",
        " _ |_| _|   " => "9",
        _ => "?",
    }
    .to_string()
}

fn parse_row(lines: &[&str]) -> String {
    let chunks0 = lines[0].bytes().collect::<Vec<_>>();
    let chunks0 = chunks0.chunks(3);
    let chunks1 = lines[1].bytes().collect::<Vec<_>>();
    let chunks1 = chunks1.chunks(3);
    let chunks2 = lines[2].bytes().collect::<Vec<_>>();
    let chunks2 = chunks2.chunks(3);
    let chunks3 = lines[3].bytes().collect::<Vec<_>>();
    let chunks3 = chunks3.chunks(3);
    let zip = chunks0.zip(chunks1).zip(chunks2).zip(chunks3);
    zip.map(|(((a, b), c), d)| {
        vec![
            std::str::from_utf8(a).unwrap(),
            std::str::from_utf8(b).unwrap(),
            std::str::from_utf8(c).unwrap(),
            std::str::from_utf8(d).unwrap(),
        ]
        .join("")
    })
    .map(|lines| parse_num(&lines))
    .collect::<Vec<_>>()
    .join("")
}

fn parse_rows(lines: &[&str]) -> String {
    lines
        .chunks(4)
        .map(|row| parse_row(row))
        .collect::<Vec<_>>()
        .join(",")
}
