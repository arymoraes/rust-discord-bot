pub fn soma(message_content: &str, keyword: &str) -> String {
    let numbers = parse_input(message_content, keyword);
    let mut sum: f32 = 0.0;

    for n in numbers {
        match n.parse::<f32>() {
            Ok(num) => sum = sum + num,
            Err(e) => {
                println!("Some user used an invalid number, {}", e);
                continue;
            }
        }
    }

    sum = (sum * 100.0).round() / 100.0;

    return sum.to_string();
}

fn parse_input<'a>(input: &'a str, keyword: &'a str) -> Vec<String> {
    let new_input = input
        .replace(keyword, "")
        .replace(",", ".")
        .split(" ")
        .map(|s| s.to_string())
        .collect();
    return new_input;
}
