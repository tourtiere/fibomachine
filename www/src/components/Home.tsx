import { Editor } from "components/Editor/Editor";
import OutputPanel from "components/OutputPanel";
import type * as Wasm from "fibomachine";
import { h } from "preact";
import { useContext, useEffect, useRef, useState } from "preact/hooks";
import { FiboContext } from "providers/fibo";

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
            <Editor
                wasmLoaded={wasmLoaded}
                onChange={(value: string) => {
                    if (value === "") return;
                    try {
                        const result = wasm.current?.run_wasm(value, 100);
                        setResult(JSON.parse(result));
                    } catch {
                        setResult({ success: false, errorType: "Other", range: undefined });

                        console.log("error");
                    }
                }}
            />
            <p>OUTPUT:</p>
            <OutputPanel />
        </div>
    );
};
