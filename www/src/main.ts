import App from "./App.svelte";

// @ts-ignore
import wasm from "../../Cargo.toml";

async function loadWasm() {
    const exports: any = await wasm();
    console.log(exports.run("a(n-1)+a(n-2);0,1", 12));
}
loadWasm();

const app = new App({
    target: document.body,
    props: {
        name: "world",
    },
});
export default app;
