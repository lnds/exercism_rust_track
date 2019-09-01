pub fn verse(n: i32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        3 ..= 99 => format!("{b} bottles of beer on the wall, {b} bottles of beer.\nTake one down and pass it around, {c} bottles of beer on the wall.\n", b=n, c=n-1),
        _ => String::new()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
