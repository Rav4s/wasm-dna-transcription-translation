import * as wasm from "wasm-dna-transcription-translation";

const urlParams = new URLSearchParams(window.location.search);
const strand = urlParams.get('strand');

console.log(strand)

wasm.main();
