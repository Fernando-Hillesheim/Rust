use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Huffman {
    Leaf { c: char, freq: u32 },
    Node { freq: u32, left: Box<Huffman>, right: Box<Huffman> },
}

#[derive(Eq, PartialEq)]
struct HeapNode {
    freq: u32,
    tree: Box<Huffman>,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn construir_arvore(frequencias: &HashMap<char, u32>) -> Box<Huffman> {
    let mut heap = BinaryHeap::new();
    for (&c, &freq) in frequencias {
        heap.push(HeapNode {
            freq,
            tree: Box::new(Huffman::Leaf { c, freq }),
        });
    }
    while heap.len() > 1 {
        let n1 = heap.pop().unwrap();
        let n2 = heap.pop().unwrap();
        heap.push(HeapNode {
            freq: n1.freq + n2.freq,
            tree: Box::new(Huffman::Node {
                freq: n1.freq + n2.freq,
                left: n1.tree,
                right: n2.tree,
            }),
        });
    }
    heap.pop().unwrap().tree
}

fn escrever_bits(writer: &mut BufWriter<File>, bits: &[bool]) -> std::io::Result<()> {
    let mut byte = 0u8;
    let mut count = 0;
    for &bit in bits {
        if bit {
            byte |= 1 << (7 - count);
        }
        count += 1;
        if count == 8 {
            writer.write_all(&[byte])?;
            byte = 0;
            count = 0;
        }
    }
    if count > 0 {
        writer.write_all(&[byte])?;
    }
    Ok(())
}

fn compactar(arquivo_entrada: &str, arquivo_saida: &str) -> std::io::Result<()> {
    let mut arquivo = File::open(arquivo_entrada)?;
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;
    let mut frequencias = HashMap::new();
    for c in conteudo.chars() {
        *frequencias.entry(c).or_insert(0) += 1;
    }
    let huffman = construir_arvore(&frequencias);
    let mut codigos = HashMap::new();
    gerar_codigos(&huffman, vec![], &mut codigos);
    let mut writer = BufWriter::new(File::create(arquivo_saida)?);
    let n = frequencias.len() as u16;
    let t = conteudo.len() as u32;
    writer.write_all(&n.to_be_bytes())?;
    writer.write_all(&t.to_be_bytes())?;
    for (&c, &f) in &frequencias {
        let c_bytes = (c as u32).to_be_bytes();
        writer.write_all(&c_bytes)?;
        writer.write_all(&f.to_be_bytes())?;
    }
    let mut bits = Vec::new();
    for c in conteudo.chars() {
        bits.extend(codigos[&c].clone());
    }
    escrever_bits(&mut writer, &bits)?;
    Ok(())
}

fn gerar_codigos(huffman: &Huffman, prefixo: Vec<bool>, mapa: &mut HashMap<char, Vec<bool>>) {
    match huffman {
        Huffman::Leaf { c, .. } => {
            mapa.insert(*c, prefixo);
        }
        Huffman::Node { left, right, .. } => {
            let mut esq = prefixo.clone();
            esq.push(false);
            gerar_codigos(left, esq, mapa);
            let mut dir = prefixo;
            dir.push(true);
            gerar_codigos(right, dir, mapa);
        }
    }
}

fn ler_u16(reader: &mut BufReader<File>) -> std::io::Result<u16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

fn ler_u32(reader: &mut BufReader<File>) -> std::io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

fn descompactar(arquivo_entrada: &str, arquivo_saida: &str) -> std::io::Result<()> {
    let mut reader = BufReader::new(File::open(arquivo_entrada)?);
    let n = ler_u16(&mut reader)?;
    let t = ler_u32(&mut reader)?;
    let mut frequencias = HashMap::new();
    for _ in 0..n {
        let mut c_buf = [0u8; 4];
        reader.read_exact(&mut c_buf)?;
        let c = std::char::from_u32(u32::from_be_bytes(c_buf)).unwrap();
        let f = ler_u32(&mut reader)?;
        frequencias.insert(c, f);
    }
    let raiz = construir_arvore(&frequencias);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let mut resultado = String::new();
    let mut atual = &raiz;
    let mut total_lidos = 0;
    for byte in buffer {
        for i in 0..8 {
            let bit = (byte >> (7 - i)) & 1;
            atual = match atual {
                Huffman::Leaf { c, .. } => {
                    resultado.push(*c);
                    total_lidos += 1;
                    if total_lidos == t {
                        break;
                    }
                    &raiz
                }
                Huffman::Node { left, right, .. } => {
                    if bit == 0 { left } else { right }
                }
            };
        }
        if total_lidos == t {
            break;
        }
    }
    let mut arquivo = File::create(arquivo_saida)?;
    arquivo.write_all(resultado.as_bytes())?;
    Ok(())
}

fn main() {
    let entrada = "compactado.huff";
    let saida = "descompactado.txt";
    match descompactar(entrada, saida) {
        Ok(_) => println!("Arquivo descompactado com sucesso."),
        Err(e) => eprintln!("Erro ao descompactar: {}", e),
    }
}
