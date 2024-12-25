use std::fmt::Debug;

fn main() {
    #[derive(Debug)]
    enum Language{
        English,
        Spanish,
        Russian,
        Japanese
    }

    let language = Language::Russian;

    match language {
        Language::English => println!("Hello World"),
        Language::Spanish => println!("Hola Mundo"),
        Language::Japanese => println!("こんにちは世界"),
        lang => println!("{:?} is not supported", lang)
    }
}
