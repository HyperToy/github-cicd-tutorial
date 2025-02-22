fn main() {
    println!("Hello, world!");
    println!("{} is {} number", 5, even_or_odd(5));
}

fn even_or_odd(number: i32) -> String {
    match number % 2 {
        0 => "even",
        _ => "odd",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use crate::even_or_odd;

    #[test]
    fn it_works() {
        assert_eq!(even_or_odd(10), "even".to_string());
    }
}
