import { h } from "preact";

import { Editor } from "../Editor/Editor";
import { useEffect, useRef, useState } from "preact/hooks";

export default () => {
    const wasm = useRef(null);
    useEffect(() => {
        import("fibomachine")
            .then((loaded) => {
                wasm.current = loaded;
            })
            .catch((e) => console.error("Error importing `index.js`:", e));
    });
    return (
        <div className={"home-page"}>
            <Editor
                onChange={(value: string) => {
                    if (value === "") return;
                    try {
                        console.log(wasm.current.run(value, 10));
                    } catch {
                        console.log("error");
                    }
                }}
            />
        </div>
    );
};
