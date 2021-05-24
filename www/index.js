import * as wasm from "fibomachine";

let timer = performance.now();
//for (let i = 0; i < 1000; i++) {
let result = wasm.run("a(n-1)+a(n-2);0", 100);
console.log(result);
let json = JSON.parse(result);
let date = performance.now();
console.log(date - timer, date);
console.log(json);
timer = date;
//}
