import { Editor } from "components/Editor/Editor";
import OutputPanel from "components/OutputPanel/OutputPanel";
import type * as Wasm from "fibomachine";
import { h } from "preact";
import { useContext, useEffect, useRef, useState } from "preact/hooks";
import { FiboContext } from "providers/fibo";
import "../../static/GitHub_Logo.png";

export default () => {
    const wasm = useRef<typeof Wasm>(null);
    const { setResult } = useContext(FiboContext);
    const [wasmLoaded, setWasmLoaded] = useState(false);
    useEffect(() => {
        import("fibomachine")
            .then((loaded) => {
                wasm.current = loaded;
                setWasmLoaded(true);
            })
            .catch((e) => console.error("Error importing fibomachine:", e));
    });

    return (
        <div className={"home-page"}>
            <div class="header">
                <a href="/?i=a(n-1)%20%2B%20a(n-1)%20%3B0%2C1">Fibonacci sequence</a>
                <a href="/?i=if(a(n-1)%252%3D0%2C%20a(n-1)%2F2%2C%20a(n-1)*3%2B1)%20%3B19">
                    Collatz conjecture
                </a>

                <div style="flex-grow:1;"></div>
                <a href="https://github.com/tourtiere/fibomachine" class="github">
                    <img src="/GitHub_Logo.png" />
                </a>
            </div>
            <Editor
                wasmLoaded={wasmLoaded}
                onChange={(value: string) => {
                    if (value === "") {
                        setResult({ success: true, values: [] });
                    }
                    try {
                        const result = wasm.current?.run_wasm(value, 100);
                        setResult(JSON.parse(result));
                    } catch {
                        setResult({ success: false, errorType: "Other", range: undefined });
                        console.log("error");
                    }
                }}
            />
            <OutputPanel />
        </div>
    );
};
