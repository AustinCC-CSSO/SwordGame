
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

let cachedTextEncoder = new TextEncoder('utf-8', { ignoreBOM: true, fatal: true });

let cachegetUint8Memory0 = null;
function getUint8Memory() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function pollString(ptr) {
    const mem = getUint8Memory();
    const len = mem.slice(ptr).findIndex(byte => byte === 0);
    return cachedTextDecoder.decode(mem.subarray(ptr, ptr + len));
}

function getString(ptr) {
    const mem = getUint8Memory();
    const len = mem.slice(ptr).findIndex(byte => byte === 0);
    const result = cachedTextDecoder.decode(mem.subarray(ptr, ptr + len))
    deallocString(ptr, result)
    return result;
}

function writeString(ptr, str) {
    let mem = getUint8Memory();
    mem.set(cachedTextEncoder.encode(str), ptr);
    mem[ptr + str.length] = 0;
}

function deallocString(ptr, str) {
    wasm.dealloc(ptr, str.length + 1);
}

function allocString(str) {
    const ptr =  wasm.alloc(str.length + 1);
    writeString(ptr, str);

    return ptr;
}

export function greet2(str) {
    const ptr = allocString(str);

    const result = wasm.greet2(ptr);

    console.log(getString(result));

    deallocString(ptr, str);

}

export function greet() {
    wasm.greet();
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
    const imports = {};
    imports.env = {};
    imports.env.alert = function(arg0) {
        alert(getString(arg0));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;

    return wasm;
}

export default init;

