use super::process;

#[test]
fn sample_message() {
    assert_eq!(
        process(String::from("hello MATMEX")),
        "hello\nhello MATMEX rust\nhello MATMEX rust\n"
    );
}

#[test]
fn single_word_message() {
    assert_eq!(
        process(String::from("service")),
        "service\nservice rust\nservice rust\n"
    );
}

#[test]
fn first_of_several_words() {
    assert_eq!(
        process(String::from("Rust спасает продакшен")),
        "Rust\nRust спасает продакшен rust\nRust спасает продакшен rust\n"
    );
}

#[test]
fn empty_message() {
    assert_eq!(process(String::new()), "\n rust\n rust\n");
}

#[test]
fn whitespace_only_message() {
    assert_eq!(process(String::from(" \t")), "\n \t rust\n \t rust\n");
}

#[test]
fn leading_whitespace_is_preserved() {
    assert_eq!(
        process(String::from("  api is down")),
        "api\n  api is down rust\n  api is down rust\n"
    );
}

#[test]
fn trailing_whitespace_is_preserved() {
    assert_eq!(
        process(String::from("api down  ")),
        "api\napi down   rust\napi down   rust\n"
    );
}

#[test]
fn tabs_separate_words() {
    assert_eq!(
        process(String::from("gateway\tfailed")),
        "gateway\ngateway\tfailed rust\ngateway\tfailed rust\n"
    );
}

#[test]
fn unicode_message() {
    assert_eq!(
        process(String::from("продакшен падает")),
        "продакшен\nпродакшен падает rust\nпродакшен падает rust\n"
    );
}

#[test]
fn punctuation_stays_attached_to_first_word() {
    assert_eq!(
        process(String::from("ALERT: сервер упал")),
        "ALERT:\nALERT: сервер упал rust\nALERT: сервер упал rust\n"
    );
}
