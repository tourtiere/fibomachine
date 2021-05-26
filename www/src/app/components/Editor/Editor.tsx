import { EditorState, EditorView } from "@codemirror/basic-setup";
import { LanguageSupport } from "@codemirror/language";
import { language, highlight } from "./language";
import { customLinter } from "./customLinter";
import { keymap } from "@codemirror/view";
import { placeholder } from "@codemirror/view";
import { useEffect, useRef, useState } from "preact/hooks";
import { h, JSX } from "preact";
import { ViewUpdate } from "@codemirror/view";

interface Props {
    onChange: (value: string) => void;
}
export function Editor({ onChange }: Props): JSX.Element {
    const codemirrorRef = useRef(null);
    const [isloaded, setLoaded] = useState(false);

    useEffect(() => {
        if (isloaded) return;
        document.title = `You clicked imes`;
        let editor = new EditorView({
            state: EditorState.create({
                extensions: [
                    keymap.of([{ key: "Enter", run: () => null, preventDefault: true }]),
                    highlight,
                    new LanguageSupport(language),
                    customLinter,
                    placeholder("Enter your expression"),
                    EditorView.updateListener.of((v: ViewUpdate) => {
                        if (!v.docChanged) return;
                        const text = v.state.doc.sliceString(0);
                        onChange(text);
                    }),
                ],
            }),
            parent: codemirrorRef.current,
        });
        editor.focus();
        setLoaded(true);
    });

    return <div ref={codemirrorRef} />;
}
