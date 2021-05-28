import { EditorState, EditorView } from "@codemirror/basic-setup";
import { LanguageSupport } from "@codemirror/language";
import { keymap, placeholder, ViewUpdate } from "@codemirror/view";
import { h, JSX } from "preact";
import { useEffect, useRef } from "preact/hooks";
import { customLinter } from "./customLinter";
import "./editor.scss";
import { highlight, language } from "./language";

function decodeURIComponentSafe(s: string) {
    return decodeURIComponent(s.replace(/%(?![0-9][0-9a-fA-F]+)/g, "%25"));
}
function extractUrl() {
    const url = new URL(window.location.href);
    const paramRaw = url.searchParams.get("i");
    if (!paramRaw) return "";
    return decodeURIComponentSafe(paramRaw);
}

function saveUrl(value: string) {
    return window.history.pushState(
        value,
        "Fibomachine",
        value == `` ? "" : `?i=${encodeURIComponent(value)}`
    );
}

interface Props {
    onChange: (value: string) => void;
    wasmLoaded: boolean;
}

//[your expresssion] ; [first term] , [second term] ...
export function Editor({ onChange, wasmLoaded }: Props): JSX.Element {
    const codemirrorRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (!wasmLoaded) return;

        const initValue = extractUrl();
        if (initValue !== "") onChange(initValue);
        document.title = `You clicked imes`;
        let editor = new EditorView({
            state: EditorState.create({
                doc: initValue,
                extensions: [
                    keymap.of([{ key: "Enter", run: () => false, preventDefault: true }]),
                    highlight,
                    new LanguageSupport(language),
                    customLinter,
                    placeholder("Enter your expression"),
                    EditorView.updateListener.of((v: ViewUpdate) => {
                        if (!v.docChanged) return;
                        const text = v.state.doc.sliceString(0);
                        saveUrl(text);
                        onChange(text);
                    }),
                ],
            }),
            parent: codemirrorRef.current,
        });
        editor.focus();
    }, [wasmLoaded]);

    return <div ref={codemirrorRef} class="editor-wrapper" />;
}
