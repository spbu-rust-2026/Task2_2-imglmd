fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(_text: String) -> String {
    let first_word = _text.split_whitespace().next().unwrap_or("").to_string();

    let mut text = _text;

    let mut result = String::new();

    result.push_str(&first_word);
    result.push('\n');

    {
        let text_mut = &mut text;
        text_mut.push_str(" rust");

        result.push_str(text_mut);
        result.push('\n');
    }

    {
        let text_ref = &text;
        result.push_str(text_ref);
        result.push('\n');
    }

    result
}
#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
