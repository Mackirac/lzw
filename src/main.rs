#![allow(dead_code)]

extern crate bit_vec;

use bit_vec::BitVec;
use std::iter::repeat;
use std::collections::HashMap;

type EDict = HashMap<(usize, usize), usize>;
type DDict = HashMap<usize, (usize, usize)>;

fn bin (n: usize, len: usize) -> Result<BitVec, String> {
    let b = format!("{:b}", n);

    if b.len() > len {
        return Err(format!(
            "It is not possible to represent {} with a {} digits binary number.",
            n, len
        ))
    }
    Ok(
        b.chars()
            .map(|b| if b == '1' { true } else { false })
            .rev().chain(
                repeat(false).take(len - b.len())
            )
            .collect::<Vec<_>>().into_iter().rev()
        .collect()
    )
}

fn dec (bin: BitVec) -> usize {
    let mut dec = 0;
    let mut bin = bin.into_iter().rev().enumerate();
    loop {
        match bin.next() {
            Some((idx, v)) => if v { dec += 2_usize.pow(idx as u32) },
            None => break
        };
    }
    dec
}

fn encode (input: &Vec<u8>) -> Vec<u8> {
    if input.len() == 0 { return vec!(); }
    let mut output = vec!();
    let mut dict = EDict::new();
    let mut code = 256_usize;
    let mut curr = input[0] as usize;

    for idx in 0..input.len() {
        if let Some(next) = input.get(idx+1).map(|n| *n as usize) {
            let key = (curr, next);
            if let Some(code) = dict.get(&key) {
                curr = *code;
            }
            else {
                dict.insert(key, code);
                output.push(curr);
                curr = next;
                code += 1;
            }
        }
        else { output.push(curr); }
    }

    let len = (256. + dict.len() as f64).log2().ceil() as usize;
    output.into_iter()
        .map(|n| bin(n, len).unwrap())
        .fold(BitVec::new(), |mut out, mut curr| {
            out.append(&mut curr);
            out
        })
    .to_bytes()
}

fn expand (n: usize, map: &DDict) -> Vec<u8> {
    if let Some(n) = map.get(&n) {
        let mut output = vec!();
        output.append(&mut expand(n.0, map));
        output.append(&mut expand(n.1, map));
        return output
    }
    vec!(n as u8)
}

fn decode (input: &Vec<usize>) -> Vec<u8> {
    if input.len() == 0 { return vec!(); }
    let mut output = vec!();
    let mut dict = DDict::new();
    let mut code = 256_usize;
    let mut curr = input[0] as usize;
    for idx in 0..input.len() {
        if let Some(next) = input.get(idx+1).map(|n| *n as usize) {
            let key = (curr, expand(next, &dict)[0] as usize);
            dict.insert(code, key);
            output.append(&mut expand(curr, &dict));
            curr = next;
            code += 1;
        }
        else { output.append(&mut expand(curr, &dict)); }
    }
    output
}

fn main() {
    let input = vec!(10, 12, 10, 12, 15, 26, 10, 12, 15, 26);
    let output = encode(&input);
    // let _input = decode(&output);

    println!("Tamanho da entrada: {} bits.", input.len() * 8);
    println!("Entrada: {:?}\n", input);
    println!("Tamanho da saída: {} bits.", output.len() * 9);
    println!("Saída: {:?}\n", output);
    // println!("Saída decodificada: {:?}", _input);
}
