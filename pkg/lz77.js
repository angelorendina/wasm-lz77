
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
*/
export function run() {
    wasm.run();
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {Uint8Array} symbols
* @returns {Data}
*/
export function encode_8(symbols) {
    var ptr0 = passArray8ToWasm0(symbols, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.encode_8(ptr0, len0);
    return Data.__wrap(ret);
}

/**
* @param {Uint8Array} symbols
* @returns {Data}
*/
export function encode_16(symbols) {
    var ptr0 = passArray8ToWasm0(symbols, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.encode_16(ptr0, len0);
    return Data.__wrap(ret);
}

/**
* @param {Uint8Array} symbols
* @returns {Data}
*/
export function encode_32(symbols) {
    var ptr0 = passArray8ToWasm0(symbols, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.encode_32(ptr0, len0);
    return Data.__wrap(ret);
}

/**
*/
export class Data {

    static __wrap(ptr) {
        const obj = Object.create(Data.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_data_free(ptr);
    }
    /**
    * @returns {number}
    */
    get address() {
        var ret = wasm.__wbg_get_data_address(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set address(arg0) {
        wasm.__wbg_set_data_address(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get length() {
        var ret = wasm.__wbg_get_data_length(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set length(arg0) {
        wasm.__wbg_set_data_length(this.ptr, arg0);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_log_a4d4a320576c1ffe = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

