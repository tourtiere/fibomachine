import * as wasm from "fibomachine";

let timer = performance.now();
for (let i = 0; i < 1000; i++) {
    let result = wasm.run("a(n-1)+a(n-2);0,1");
    let date = performance.now();
    console.log(date - timer, result);
    timer = date;
}
