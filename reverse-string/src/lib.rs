pub fn reverse(input: &str) -> String {
    let mut input = String::from(input);
    let mut reversed = String::new();

    loop {
        match input.pop() {
            None => break,
            Some(char) => reversed.push(char)
        }
    }

    reversed
}
