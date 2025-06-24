use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::{File};
use std::io::{Read, Write, BufWriter, BufReader};

#[derive(Debug, Clone)]
enum Huffman {
    Leaf { c: char, freq: u32 },
    Node { freq: u32, left: Box<Huffman>, right: Box<Huffman> },
}

//wrapper para implementar os traits de ordenação
#[derive(Debug)]
struct HeapNode(Huffman);

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.freq() == other.0.freq()
    }
}
impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.0.freq().cmp(&self.0.freq())) // min-heap
    }
}
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.freq().cmp(&self.0.freq())
    }
}


impl Huffman {
    //retorna a frequencia assiciada a um nó
    fn freq(&self) -> u32 {
        match self {
            Huffman::Leaf { freq, .. } => *freq,
            Huffman::Node { freq, .. } => *freq,
        }
    }
    //controi a arvore usando caracteres e suas frequencias
    fn build_tree(freqs: &HashMap<char, u32>) -> Huffman {
        let mut heap = BinaryHeap::new();

        for (&c, &freq) in freqs {
            heap.push(HeapNode(Huffman::Leaf { c, freq }));
        }

        while heap.len() > 1 {
            let left = heap.pop().unwrap().0;
            let right = heap.pop().unwrap().0;
            let freq = left.freq() + right.freq();
            heap.push(HeapNode(Huffman::Node {
                freq,
                left: Box::new(left),
                right: Box::new(right),
            }));
        }

        heap.pop().unwrap().0
    }
    //constroi o codebook (tabela de códigos binários) para cada caractere com base na árvore.
    fn build_codebook(&self, prefix: Vec<bool>, codebook: &mut HashMap<char, Vec<bool>>) {
        match self {
            Huffman::Leaf { c, .. } => {
                codebook.insert(*c, prefix);
            }
            Huffman::Node { left, right, .. } => {
                //esquerda -> false
                let mut left_prefix = prefix.clone();
                left_prefix.push(false);
                left.build_codebook(left_prefix, codebook);
                //direita -> true
                let mut right_prefix = prefix;
                right_prefix.push(true);
                right.build_codebook(right_prefix, codebook);
            }
        }
    }
    //transforma a estrutura da árvore em um vetor de bytes para salvamento
    fn serialize(&self, out: &mut Vec<u8>) {
        match self {
            Huffman::Leaf { c, .. } => {
                out.push(1);
                out.push(*c as u8);
            }
            Huffman::Node { left, right, .. } => {
                out.push(0);
                left.serialize(out);
                right.serialize(out);
            }
        }
    }
    //reconstroi a árvore a partir de dados serializados
    fn deserialize(data: &[u8], pos: &mut usize) -> Huffman {
        let tag = data[*pos];
        *pos += 1;
        if tag == 1 {
            let c = data[*pos] as char;
            *pos += 1;
            Huffman::Leaf { c, freq: 0 }
        } else {
            let left = Box::new(Huffman::deserialize(data, pos));
            let right = Box::new(Huffman::deserialize(data, pos));
            Huffman::Node {
                freq: 0,
                left,
                right,
            }
        }
    }
    //le os bits e percorre a arvore em busca do nó folha (caractere)
    fn decode<'a>(&self, bits: &'a [bool], pos: &mut usize) -> Option<char> {
        match self {
            Huffman::Leaf { c, .. } => Some(*c),
            Huffman::Node { left, right, .. } => {
                if *pos >= bits.len() {
                    return None;
                }
                let bit = bits[*pos];
                *pos += 1;
                if !bit {
                    left.decode(bits, pos)
                } else {
                    right.decode(bits, pos)
                }
            }
        }
    }
}
//transforma a string em uma sequencia de bits usando a tabela de huffman (codebook)
fn encode_bits(text: &str, codebook: &HashMap<char, Vec<bool>>) -> Vec<bool> {
    let mut bits = Vec::new();
    for c in text.chars() {
        if let Some(code) = codebook.get(&c) {
            bits.extend(code);
        }
    }
    bits
}
//converte a sequencia de bits gerada para uma sequencia de bytes para salvamento
fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut byte = 0u8;
    for (i, bit) in bits.iter().enumerate() {
        if *bit {
            byte |= 1 << (7 - (i % 8));
        }
        if i % 8 == 7 {
            bytes.push(byte);
            byte = 0;
        }
    }
    if bits.len() % 8 != 0 {
        bytes.push(byte);
    }
    bytes
}
//converte os bytes do arquivo para um vetor de bits para o codigo de descompactação
fn bytes_to_bits(bytes: &[u8], bit_len: usize) -> Vec<bool> {
    let mut bits = Vec::new();
    for byte in bytes {
        for i in 0..8 {
            if bits.len() == bit_len {
                return bits;
            }
            bits.push((byte >> (7 - i)) & 1 == 1);
        }
    }
    bits
}

fn compactar(input_file: &str, output_file: &str) {
    let mut file = File::open(input_file).expect("Não foi possível abrir o arquivo de entrada");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut freq = HashMap::new();
    for c in content.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let tree = Huffman::build_tree(&freq);
    let mut codebook = HashMap::new();
    tree.build_codebook(Vec::new(), &mut codebook);

    let bits = encode_bits(&content, &codebook);
    let bytes = bits_to_bytes(&bits);

    let mut tree_data = Vec::new();
    tree.serialize(&mut tree_data);

    let mut out = BufWriter::new(File::create(output_file).unwrap());
    out.write_all(&(tree_data.len() as u32).to_be_bytes()).unwrap();
    out.write_all(&(bits.len() as u32).to_be_bytes()).unwrap();
    out.write_all(&tree_data).unwrap();
    out.write_all(&bytes).unwrap();
}

fn descompactar(input_file: &str, output_file: &str) {
    let mut file = BufReader::new(File::open(input_file).unwrap());
    let mut len_buf = [0u8; 4];
    file.read_exact(&mut len_buf).unwrap();
    let tree_len = u32::from_be_bytes(len_buf) as usize;

    file.read_exact(&mut len_buf).unwrap();
    let bit_len = u32::from_be_bytes(len_buf) as usize;

    let mut tree_data = vec![0u8; tree_len];
    file.read_exact(&mut tree_data).unwrap();
    let mut pos = 0;
    let tree = Huffman::deserialize(&tree_data, &mut pos);

    let mut compressed_data = Vec::new();
    file.read_to_end(&mut compressed_data).unwrap();
    let bits = bytes_to_bits(&compressed_data, bit_len);

    let mut result = String::new();
    let mut i = 0;
    while i < bits.len() {
        if let Some(c) = tree.decode(&bits, &mut i) {
            result.push(c);
        } else {
            break;
        }
    }

    let mut out = File::create(output_file).unwrap();
    out.write_all(result.as_bytes()).unwrap();
}

fn main() {
    compactar("input.txt", "compactado.bin");
    descompactar("compactado.bin", "descompactado.txt");
}
