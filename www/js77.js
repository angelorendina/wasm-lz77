/** 
 * Returns the position of the rightmost match of search in the given window, or -1.
 */
function rfind(window, search) {
    if (search.length > window.length) {
        return -1;
    }
    for (let position = window.length - search.length; position >= 0; position--) {
        const segment = window.slice(position, position + search.length);
        let segmentMatch = true;
        for (let i = 0; i < segment.length; i++) {
            if (segment[i] !== search[i]) {
                segmentMatch = false;
                break;
            }
        }
        if (segmentMatch) return position;
    }
    return -1;
}

/**
 * Computes the next Code, given the window and a non-empty lookahead.
 */
function findCode(window, lookahead) {
    const code = {
        offset: 0,
        length: 0,
        literal: lookahead[0]
    };
    let searchLength = 1;
    while (searchLength < lookahead.length) {
        const search = lookahead.slice(0, searchLength);
        const rightmostMatch = rfind(window, search);
        if (rightmostMatch < 0) {
            break;
        }
        code.offset = window.length - rightmostMatch;
        code.length = searchLength;
        code.literal = lookahead[searchLength];
        searchLength++;
    }
    return code;
}

/**
 * The LZ77 compression, the turning input into a sequence of Codes.
 */
function encode(input, windowSize) {
    const encoded = [];
    let position = 0;
    while (position < input.length) {
        const lookahead = input.slice(position);
        const windowStart = Math.max(position - windowSize + 1, 0);
        const window = input.slice(windowStart, position);
        const code = findCode(window, lookahead);
        position += code.length + 1;
        encoded.push(code);
    }
    return encoded;
}

function encodingToArray(encoded) {
    return encoded.reduce((array, code) => {
        return array.concat([
            code.offset,
            code.length,
            code.literal
        ])
    }, []);
}

export function jsEncode(input, bits) {
    let encoded;
    switch (bits) {
        case 8:
            encoded = encode(input, 1 << 8);
            return new Uint8Array(encodingToArray(encoded));
        case 16:
            encoded = encode(input, 1 << 16);
            return new Uint16Array(encodingToArray(encoded));
        case 32:
            encoded = encode(input, Number.MAX_SAFE_INTEGER);
            return new Uint32Array(encodingToArray(encoded));
    }
}
