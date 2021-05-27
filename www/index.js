import init, { encode_8, encode_16, encode_32 } from '../pkg/lz77.js';
import { jsEncode } from './js77.js';

function writeOutput(bits, line) {
    const output = document.getElementById(`output${bits}`);
    output.innerHTML += `<p>${line}</p>`;
}

async function run({ wasm, fileBytes, bits}) {
    const wasmEncode = [];
    wasmEncode[8] = encode_8;
    wasmEncode[16] = encode_16;
    wasmEncode[32] = encode_32;

    writeOutput(bits, `Encoding as u${bits} window size with WASM...`);
    const wasmStart = Date.now();
    const wasmEncodingData = wasmEncode[bits](fileBytes);
    const wasmTime = Date.now() - wasmStart;
    writeOutput(bits, `Done. It took ${wasmTime}ms.`);

    writeOutput(bits, `Encoding as u${bits} window size with JS...`);
    const jsStart = Date.now();
    const jsEncoding = jsEncode(fileBytes, bits);
    const jsTime = Date.now() - jsStart;
    writeOutput(bits, `Done. It took ${jsTime}ms.`);

    writeOutput(bits, 'Comparing encoding...');
    writeOutput(bits, `WASM returned ${wasmEncodingData.length} symbols.`);
    writeOutput(bits, `JS returned ${jsEncoding.length} symbols.`);
    if (wasmEncodingData.length !== jsEncoding.length) {
        writeOutput(bits, `ERROR: Different length.`);
        return;
    }
    writeOutput(bits, 'Length matches.');

    writeOutput(bits, 'Comparing symbols...');
    let wasmEncoding;
    switch (bits) {
        case 8:
            wasmEncoding = new Uint8Array(wasm.memory.buffer, wasmEncodingData.address, wasmEncodingData.length);
            break;
        case 16:
            wasmEncoding = new Uint16Array(wasm.memory.buffer, wasmEncodingData.address, wasmEncodingData.length);
            break;
        case 32:
            wasmEncoding = new Uint32Array(wasm.memory.buffer, wasmEncodingData.address, wasmEncodingData.length);
            break;
    }
    for (let i = 0; i < jsEncoding.length; i++) {
        if (jsEncoding[i] !== wasmEncoding[i]) {
            writeOutput(bits, `ERROR: Encoding mismatch.`);
            return;
        }
    }
    writeOutput(bits, 'Content matches!');
}

(async function () {
    const wasm = await init();

    writeOutput(0, 'Fetching file...');
    const fileBytes = await fetch('https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg')
        .then(response => response.blob())
        .then(blob => {
            document.getElementById('logo').src = URL.createObjectURL(blob);
            return new Response(blob).arrayBuffer();
        })
        .then(buffer => new Uint8Array(buffer, 0, buffer.byteLength));
    writeOutput(0, `Done. Got ${fileBytes.length} bytes.`);

    await run({ wasm, fileBytes, bits: 8 });
    await run({ wasm, fileBytes, bits: 16 });
    await run({ wasm, fileBytes, bits: 32 });
})();
