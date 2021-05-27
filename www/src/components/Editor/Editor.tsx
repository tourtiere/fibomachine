import { EditorState, EditorView } from "@codemirror/basic-setup";
import { LanguageSupport } from "@codemirror/language";
import { keymap, placeholder, ViewUpdate } from "@codemirror/view";
import { h, JSX } from "preact";
import { useEffect, useRef } from "preact/hooks";
import { customLinter } from "./customLinter";
import { highlight, language } from "./language";

function extractUrl() {
    const url = new URL(window.location.href);
    const paramRaw = url.searchParams.get("i");
    if (!paramRaw) return "";
    return decodeURIComponent(paramRaw);
}

function saveUrl(value: string) {
    if (value === "") {
        return window.history.pushState(value, "Fibomachine", ``);
    }
    window.history.pushState(value, "Fibomachine", `?i=${encodeURIComponent(value)}`);
}

interface Props {
    onChange: (value: string) => void;
}

//[your expresssion] ; [first term] , [second term] ...
export function Editor({ onChange }: Props): JSX.Element {
    const codemirrorRef = useRef<HTMLDivElement>(null);
    const initValue = extractUrl();
    useEffect(() => {
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
    }, []);

    return <div ref={codemirrorRef} />;
}
