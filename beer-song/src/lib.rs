pub fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        other => String::from(format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", other, other, other - 1))
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = String::new();

    let mut index = start;

    while index >= end {
        let verse_to_add: &str = &verse(index);

        song.push_str(verse_to_add);

        if index != end { song.push_str("\n") }
        if index == 0 { break };

        index -= 1;
    }

    song
}
