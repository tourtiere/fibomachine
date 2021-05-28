import { Diagnostic, linter } from "@codemirror/lint";
import type { EditorView } from "@codemirror/view";

const source = (view: EditorView) => {
    const text = view.state.doc.sliceString(0);
    const diagnotics: Diagnostic[] = [
        { from: Math.max(0, text.length - 5), to: text.length, message: "", severity: "error" },
    ];
    return [];
};
export const customLinter = linter(source);
