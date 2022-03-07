pub fn reverse(input: &str) -> String {
    if input.len() == 0 { return String::from("") }

    let mut reversed = String::new();
    let mut index = input.len() - 1;

    loop {
        reversed.push(input.chars().nth(index).unwrap());

        if index == 0 { break }

        index -= 1;
    }

    reversed
}
