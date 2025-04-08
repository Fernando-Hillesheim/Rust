fn eh_primo_forca_bruta(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limite = (n as f64).sqrt() as u64 + 1;
    for i in (3..=limite).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn filtrar_primos(numeros: Vec<u64>) -> Vec<u64> {
    numeros.into_iter().filter(|&n| eh_primo_forca_bruta(n)).collect()
}

fn main() {
    let dados = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17];
    let primos = filtrar_primos(dados);

    println!("Números primos: {:?}", primos);
}