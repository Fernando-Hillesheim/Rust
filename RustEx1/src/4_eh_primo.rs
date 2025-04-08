fn eh_primo_forca_bruta(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limite = (n as f64).sqrt() as u128 + 1;
    for i in (3..=limite).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn mod_exp(mut base: u128, mut exp: u128, modulo: u128) -> u128 {
    let mut result = 1;
    base = base % modulo;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp /= 2;
    }
    result
}

fn miller_rabin(n: u128, bases: &[u128]) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }


    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    'base_loop: for &a in bases {
        if a >= n - 1 {
            continue; 
        }
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
                continue 'base_loop;
            }
        }
        return false;
    }
    true
}

fn eh_primo_probabilistico(n: u128) -> bool {
    let bases: &[u128] = &[2, 3, 5, 7, 11, 13, 17, 19, 23];
    miller_rabin(n, bases)
}

fn main() {
    let n: u128 = 32416190071;

    println!("Força bruta: {} é primo? {}", n, eh_primo_forca_bruta(n));
    println!("Probabilístico: {} é primo? {}", n, eh_primo_probabilistico(n));
}