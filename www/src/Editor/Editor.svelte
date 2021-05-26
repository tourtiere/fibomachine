<script lang="typescript">
    import { onMount } from "svelte";
    import { EditorState, EditorView } from "@codemirror/basic-setup";
    import { LanguageSupport } from "@codemirror/language";
    import { language, highlight } from "./language";
    import { customLinter } from "./customLinter";
    import { keymap } from "@codemirror/view";
    import { placeholder } from "@codemirror/view";

    onMount(() => {
        const parent = document.querySelector("#editor");
        let editor = new EditorView({
            state: EditorState.create({
                extensions: [
                    keymap.of([{ key: "Enter", run: () => null, preventDefault: true }]),
                    highlight,
                    new LanguageSupport(language),
                    customLinter,
                    placeholder("Enter your expression"),
                ],
            }),

            parent,
        });
        editor.focus();
    });
    export let answer;
    let className;
</script>

<div id="editor" />
