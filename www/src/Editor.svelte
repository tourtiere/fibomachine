<script lang="typescript">
    import { CodeJar } from "codejar";
    import { onMount } from "svelte";

    onMount(() => {
        const editor: HTMLElement = document.querySelector(".editor");
        const rainbow = (ch: any, i: any) => {
            let red = Math.round(Math.sin(0.01 * i + 0) * 127 + 128);
            let green = Math.round(Math.sin(0.01 * i + (2 * Math.PI) / 3) * 127 + 128);
            let blue = Math.round(Math.sin(0.01 * i + (4 * Math.PI) / 3) * 127 + 128);
            return `<span style="color: rgb(${red}, ${green}, ${blue})">` + ch + `</span>`;
        };
        const lolcat = (editor: any) => {
            const code = editor.textContent.split("").map(rainbow).join("");
            editor.innerHTML = code;
        };
        const jar = CodeJar(editor, lolcat);
    });
</script>

<div class="editor" />

<style>
</style>
