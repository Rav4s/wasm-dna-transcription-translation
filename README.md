# wasm-dna-transcription-translation
DNA transcription and translation in WebAssembly using Rust
## What is DNA transcription & translation?
Transcription and translation are both part of protein synthesis, an integral process for cells. The central dogma of biology states that DNA is transcribed to RNA, which is translated into proteins. I'm not the best at explaining concepts like these, so you might want to check out [this wikipedia article](https://en.wikipedia.org/wiki/Protein_biosynthesis) 😉
## What does this do?
This program converts a DNA strand to messenger RNA (mRNA), which is then converted into amino acids.

First, it takes in a DNA strand as input and converts this strand to a complementary messenger RNA strand using these mappings:

A &#8594; U  
T &#8594; A  
C &#8594; G  
G &#8594; C  

For example, `GTACTAGAGCATTT` would be converted to `CAUGAUCUCGUAA`. After this, it looks for a start codon (AUG) and removes anything before that codon. For example, `CAUGAUCUCGUAA` would be converted to `AUGAUCUCGUAA`. Then, it turns the result of the previous step into a vector broken into 3-character items. For example, `AUGAUCUCGUAA` would be converted to `['AUG', 'AUC', 'UCG', 'UAA']`. Then, it looks for a STOP codon (UAA, UAG, UGA) and truncates the vector at that codon. Finally, it uses an awesome [match statement](https://github.com/Rav4s/wasm-dna-transcription-translation/blob/main/src/lib.rs#L109), which is basically a predefined [codon chart](https://www.google.com/search?q=codon+chart&rlz=1C1CHBF_enUS912US912&tbm=isch&source=iu&ictx=1&fir=SVhfz4tRL5GzVM%252Cx4w9lB13r4FJ7M%252C_&vet=1&usg=AI4_-kSuwWL4sbNFjTZd3fkSLRoPujadRw&sa=X&ved=2ahUKEwi7verdq-7sAhVQSK0KHUXZAp8Q9QF6BAgBEFg&biw=1366&bih=625&safe=active&ssui=on#imgrc=SVhfz4tRL5GzVM), to translate each codon to an amino acid.

## What's so special about this?
I'm glad you asked! Unlike a regular program that just transcribes and translates DNA into amino acids, this one utilizes WebAssembly, so you can do it all in a browser. The `index.html` file has a form field, which redirects to `app.html` with a query string in the form `?strand=DNAstrandHere`. The `app.js` file then parses this query string and passes it to the Rust program as input. After transcribing and translating the strand, the Rust program uses `web_sys` to insert the amino acids into the DOM.

Of course there's some CSS as well to make it all look nice ✨

## Installation
Before installing this program, you'll need a few dependencies. These include:
- An up to date Rust install
- wasm-bindgen
- wasm-pack
- node.js & NPM

## Building and running

## In action
This is available in action at https://dna-transcription-translation.yeetpc.com/
