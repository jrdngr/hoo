import init, { run_app } from './pkg/hoo_frontend.js';
async function main() {
   await init('/pkg/hoo_frontend_bg.wasm');
   run_app();
}
main()
