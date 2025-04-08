fn maior_valor(array: &[i32]) -> Option<i32> {
    if array.is_empty() {
        return None;
    }

    let mut maior = array[0];
    for &valor in array.iter().skip(1) {
        if valor > maior {
            maior = valor;
        }
    }

    Some(maior)
}


fn main() {
    let numeros = [3, 7, -2, 10, 5];
    match maior_valor(&numeros) {
        Some(valor) => println!("O maior valor é {}", valor),
        None => println!("O array está vazio"),
    }
}