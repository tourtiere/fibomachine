<script>
    import Editor from "./Editor/Editor.svelte";
    export let name;
</script>

<main>
    <div class="editor-wrapper">
        <Editor />
    </div>
    <h1>Hello {name}!</h1>
    <p>
        Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte
        apps.
    </p>
</main>

<style>
    main {
        text-align: center;
        padding: 1em;
        max-width: 240px;
        margin: 0 auto;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 4em;
        font-weight: 100;
    }

    @media (min-width: 640px) {
        main {
            max-width: none;
        }
    }
    .editor-wrapper {
        height: 31;
        padding: 0px 7px;
        font-size: 17px;
        font-family: Arial, monospace;
        border: 1px solid#888888;
        box-shadow: 2px 2px 4px #888888;
        border-radius: 5px;
    }
</style>
