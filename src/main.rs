fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(_text: String) -> String {
    todo!("исправьте владение и заимствования строки");
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
