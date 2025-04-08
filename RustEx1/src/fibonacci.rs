fn fibonacci (n:u32) -> Vec<i32> {
    let mut sequencia : Vec<i32> = Vec::with_capacity(n as usize);

    let mut x = 0;
    sequencia.push(x);

    if n == 1 {
        return sequencia;
    }
    
    let mut y = 1;
    sequencia.push(1);

    if n == 2 {
        return sequencia;
    }

    for i in 2..(n) {
        // se for um numero impar da sequencia
        if (i % 2) == 0 {
            x = x + y;
            sequencia.push(x);
        }
        else{
            y = y + x;
            sequencia.push(y);
        }
    }

    return sequencia
}

fn main () {
    let n:u32 = 20;
    let sequencia :Vec<i32> = fibonacci(n);
    println!("{:?}", sequencia);
}