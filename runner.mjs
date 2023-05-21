import fs from 'fs';

const dec = new TextDecoder("utf-8");

if (process.argv.length != 3) {
    console.log("Usage: node runner.mjs <wasm-file>");
    process.exit(0);
}

const wasmfile = process.argv[2];
if (!fs.existsSync(wasmfile)) {
    console.log("Error: File not found:", wasmfile);
    process.exit(1);
}

const wasmBuffer = fs.readFileSync(wasmfile);

async function main() {

    let memory = new ArrayBuffer(0) // will be changed after instantiate

    const imports = {
        wasi_snapshot_preview1: {
            fd_write: (fd, iovs_start, iovs_count, out_bytes_written) => {
                if (fd != 1) throw "cannot write to other FD than 1";

                const iovs = new DataView(memory, iovs_start, iovs_count * 8);

                let result = 0;
                for (let ptr = 0; ptr < iovs.byteLength; ptr += 8) {
                    const strStart = iovs.getInt32(ptr, true);
                    const length = iovs.getInt32(ptr + 4, true);

                    const str = dec.decode(new DataView(memory, strStart, length));
                    process.stdout.write(str);

                    result += length;
                }
                return result;
            },
            proc_exit: process.exit
        }
    };
    const wasmModule = await WebAssembly.instantiate(wasmBuffer, imports);
    memory = wasmModule.instance.exports.memory.buffer;

    const start = wasmModule.instance.exports._start;
    start();
}

await main();