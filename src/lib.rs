mod utils;

use wasm_bindgen::prelude::*;
use std::io;
use std::str;
use std::process;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

//#[wasm_bindgen]
pub fn transcription(dna: String) -> String {
    let char_vec: Vec<char> = dna.chars().collect();
    let mut transcribed_vec: Vec<char> = Vec::new();
    for i in char_vec {

        match i {
            'A' => transcribed_vec.push('U'),
            'T' => transcribed_vec.push('A'),
            'C' => transcribed_vec.push('G'),
            'G' => transcribed_vec.push('C'),
            _ => {
                // println!("Incorrect char");
                break;
             }
        }
    }

    let transcribed_string: String = transcribed_vec.into_iter().collect();
    return transcribed_string;
}

//#[wasm_bindgen]
pub fn find_start(messenger_rna: String) -> String {
    let start_codon = "AUG";
    let start_index = messenger_rna.find(start_codon).unwrap();
    let inter_rna: String = messenger_rna.chars().skip(start_index).collect();
    return inter_rna;
}

//#[wasm_bindgen]
pub fn break_into_codons(inter_rna: String) -> Vec<String> {
    let sub_len = 3;
    let subs = inter_rna.as_bytes()
        .chunks(sub_len)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let mut string_vec: Vec<String> = Vec::new();
    for i in &subs {
        string_vec.push(i.to_string());
    }

    return string_vec;
}

//#[wasm_bindgen]
pub fn find_stop(inter_codons: &[String]) -> usize {
    let mut stop_index_1: usize = usize::MAX;
    let mut stop_index_2: usize = usize::MAX;
    let mut stop_index_3: usize = usize::MAX;

    if inter_codons.iter().any(|i| i == "UAA") {
        stop_index_1 = inter_codons.iter().position(|r| r == "UAA").unwrap();
    }
    if inter_codons.iter().any(|i| i == "UAG") {
        stop_index_2 = inter_codons.iter().position(|r| r == "UAG").unwrap();
    }
    if inter_codons.iter().any(|i| i == "UGA") {
        stop_index_3 = inter_codons.iter().position(|r| r == "UGA").unwrap();
    }

    let stop_index = find_first(stop_index_1, stop_index_2, stop_index_3);

    return stop_index;
}

//#[wasm_bindgen]
pub fn find_first(stop_index_1: usize, stop_index_2: usize, stop_index_3: usize) -> usize {
    let mut stop_index: usize = 1;

    if stop_index_1 < stop_index_2 {
        if stop_index_1 < stop_index_3{
            println!("UAA stop codon found!");
            stop_index = stop_index_1;
        }
    }
    else if stop_index_2 < stop_index_1 {
        if stop_index_2 < stop_index_3 {
            println!("UAG stop codon found!");
            stop_index = stop_index_2;
        }
    }
    else if stop_index_3 < stop_index_1 {
        if stop_index_3 < stop_index_2 {
            println!("UGA stop codon found!");
            stop_index = stop_index_3;
        }
    }
    else {
        println!("No stop codon found!");
        process::exit(1);
    }

    return stop_index;
}

//#[wasm_bindgen]
pub fn translation(inter_codons: Vec<String>) -> Vec<String> {
    let mut amino_acids_list: Vec<String> = Vec::new();

    for i in inter_codons {
        match i.as_str() {
             "GUU" | "GUC" | "GUA" | "GUG" => amino_acids_list.push("Valine".to_string()),
             "GCU" | "GCC" | "GCA" | "GCG" => amino_acids_list.push("Alanine".to_string()),
             "GAU" | "GAC" => amino_acids_list.push("Aspartic Acid".to_string()),
             "GAA" | "GAG" => amino_acids_list.push("Glutamic Acid".to_string()),
             "GGU" | "GGC" | "GGA" | "GGG" => amino_acids_list.push("Glycine".to_string()),
             "UUU" | "UUC" => amino_acids_list.push("Phenylalanine".to_string()),
             "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => amino_acids_list.push("Leucine".to_string()),
             "UCU" | "UCC" | "UCA" | "UCG" | "AGU" | "AGC" => amino_acids_list.push("Serine".to_string()),
             "UAU" | "UAC" => amino_acids_list.push("Tyrosine".to_string()),
             "UAA" | "UAG" => amino_acids_list.push("STOP".to_string()),
             "UGU" | "UGC" => amino_acids_list.push("Cysteine".to_string()),
             "UGA" => amino_acids_list.push("STOP".to_string()),
             "UGG" => amino_acids_list.push("Tryptophan".to_string()),
             "CCU" | "CCC" | "CCA" | "CCG" => amino_acids_list.push("Proline".to_string()),
             "CAU" | "CAC" => amino_acids_list.push("Histidine".to_string()),
             "CAA" | "CAG" => amino_acids_list.push("Glutamine".to_string()),
             "CGU" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => amino_acids_list.push("Arginine".to_string()),
             "AUU" | "AUC" | "AUA" => amino_acids_list.push("Isoleucine".to_string()),
             "AUG" => amino_acids_list.push("Methionine".to_string()),
             "ACU" | "ACC" | "ACA" | "ACG" => amino_acids_list.push("Threonine".to_string()),
             "AAU" | "AAC" => amino_acids_list.push("Asparginine".to_string()),
             "AAA" | "AAG" => amino_acids_list.push("Lysine".to_string()),
             _ => {
                // println!("Incorrect char");
                break;
             }
        }
    }

    return amino_acids_list;
}

#[wasm_bindgen]
pub fn main() {
    println!("Enter the DNA strand to be transcribed and translated: ");

    let mut strand: String = "TACATGCCATACGAGACGAGCGCGCCTAAGCGGCGCAGACTCATGGTCATT".to_string();
    strand = strand.to_uppercase();

    let messenger_rna = transcription(strand);
    println!("The transcribed strand is: {}", messenger_rna);
    let inter_rna = find_start(messenger_rna);
    println!("{}", inter_rna);
    let mut inter_codons = break_into_codons(inter_rna);
    let mut stop_index = find_stop(&inter_codons);
    stop_index = stop_index + 1;
    println!("{}", stop_index);
    inter_codons.truncate(stop_index);
    let amino_acids_list = translation(inter_codons);
    print!("The translated amino acids are: ");
    for i in amino_acids_list {
        print!("{}, ", i);
        alert(&i);
    }
}
