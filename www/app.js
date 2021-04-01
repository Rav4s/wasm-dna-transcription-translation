import * as wasm from "wasm-dna-transcription-translation";

const urlParams = new URLSearchParams(window.location.search);
const strand = urlParams.get('strand');

try {
  wasm.main(strand);
} catch(err) {
   alert('Oh no! Something went wrong. Please check that your DNA strand only uses valid characters, and has a stop & start codon. You will now be redirected to the homepage.')
   window.location.href = '/';
}
