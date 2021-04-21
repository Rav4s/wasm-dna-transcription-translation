// Copyright Ravi Shah 2021
// You should've got the GPL v3.0 with this software
// If ya didn't, it's available here:
// https://www.gnu.org/licenses/gpl-3.0.en.html

// Imports

mod utils;

use wasm_bindgen::prelude::*;
use std::str;
use std::process;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn transcription(dna: String) -> String {
    let char_vec: Vec<char> = dna.chars().collect();
    let mut transcribed_vec: Vec<char> = Vec::new();
    for i in char_vec {

        // Basically converts DNA strand to mRNA

        match i {
            'A' => transcribed_vec.push('U'),
            'T' => transcribed_vec.push('A'),
            'C' => transcribed_vec.push('G'),
            'G' => transcribed_vec.push('C'),
            _ => {
                break;
             }
        }
    }

    // Collect transcribed mRNA into a String
    let transcribed_string: String = transcribed_vec.into_iter().collect();
    return transcribed_string;
}

pub fn find_start(messenger_rna: String) -> String {
    let start_codon = "AUG";
    // Find index of start codon
    let start_index = messenger_rna.find(start_codon).unwrap();
    // Remove anything before start codon
    let inter_rna: String = messenger_rna.chars().skip(start_index).collect();
    return inter_rna;
}

pub fn break_into_codons(inter_rna: String) -> Vec<String> {
    let sub_len = 3;
    // Split into codons (chunks of 3 bases each)
    let subs = inter_rna.as_bytes()
        .chunks(sub_len)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    // Combine into a vector
    let mut string_vec: Vec<String> = Vec::new();
    for i in &subs {
        string_vec.push(i.to_string());
    }

    return string_vec;
}

pub fn find_stop(inter_codons: &[String]) -> usize {
    let mut stop_index_1: usize = usize::MAX;
    let mut stop_index_2: usize = usize::MAX;
    let mut stop_index_3: usize = usize::MAX;

    // Find position of any stop codons
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

pub fn find_first(stop_index_1: usize, stop_index_2: usize, stop_index_3: usize) -> usize {
    let mut stop_index: usize = 1;

    // Find which stop codon actually occured first
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

pub fn translation(inter_codons: Vec<String>) -> Vec<String> {
    let mut amino_acids_list: Vec<String> = Vec::new();

    // Matching codons to amino acids
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
                break;
             }
        }
    }

    return amino_acids_list;
}

#[wasm_bindgen]
pub fn main(mut strand: String) {
    println!("Enter the DNA strand to be transcribed and translated: ");

    strand = strand.to_string().to_uppercase(); // Convert to uppercase

    let messenger_rna = transcription(strand); // Transcribe strand
    println!("The transcribed strand is: {}", messenger_rna);
    let inter_rna = find_start(messenger_rna); // Find start codon
    println!("{}", inter_rna);
    let mut inter_codons = break_into_codons(inter_rna); // Break into codons
    let mut stop_index = find_stop(&inter_codons); // Find index of stop codon
    stop_index = stop_index + 1;
    println!("{}", stop_index);
    inter_codons.truncate(stop_index); // Truncate at stop index
    let amino_acids_list = translation(inter_codons); // Translate to amino acids

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let codons_div = document.get_element_by_id("codons").expect("codons div should exist"); // Get codons div

    // Insert codons header into the DOM
    let val = document.create_element("h2").unwrap();
    val.set_text_content(Some("Codons:"));
    codons_div.append_with_node_1(&val).unwrap();

    print!("The translated amino acids are: ");
    // Insert amino acids into the DOM
    for i in amino_acids_list {
        let val = document.create_element("p").unwrap();
        val.set_text_content(Some(&i));
        codons_div.append_with_node_1(&val).unwrap();
    }
}
