fn eh_palindromo_limpo(s: &str) -> bool {
    let filtrada: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    filtrada.chars().eq(filtrada.chars().rev())
}

fn main() {
    let exemplo1 = "Ame a ema";
    let exemplo2 = "Roma me tem amor";
    let exemplo3 = "Rust não é palíndromo";

    println!("'{}' é palíndromo? {}", exemplo1, eh_palindromo_limpo(exemplo1));
    println!("'{}' é palíndromo? {}", exemplo2, eh_palindromo_limpo(exemplo2));
    println!("'{}' é palíndromo? {}", exemplo3, eh_palindromo_limpo(exemplo3));
}