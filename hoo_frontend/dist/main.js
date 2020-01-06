import init, { run } from './pkg/package.js';
async function main() {
   await init('/pkg/package_bg.wasm');
   run();
}
main()