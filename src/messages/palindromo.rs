pub fn palindromo(input: &str, keyword: &str) -> String {
    let new_input = parse_input(input, keyword);
    let char_vector: Vec<&str> = new_input.split("").collect();

    return char_vector
        .into_iter()
        .rev()
        .map(|s| s.to_string())
        .collect();
}

fn parse_input<'a>(input: &'a str, keyword: &'a str) -> String {
    let new_input = input.replace(keyword, "");
    return new_input;
}
