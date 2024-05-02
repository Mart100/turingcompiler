<script lang="ts">
    import hljs from "highlight.js/lib/core";

    import "highlight.js/styles/vs2015.css";
    import javascript from "highlight.js/lib/languages/javascript";
    import { interpreter } from "../utils/Interpreter";

    hljs.registerLanguage("javascript", javascript);

    let programCode = "";

    interpreter.originalProgram.subscribe((value) => {
        programCode = hljs.highlight(value, {
            language: "javascript",
            ignoreIllegals: true,
        }).value;
    });
</script>

<div class="program">
    <h2>Original Program</h2>
    <pre class="code">{@html programCode}</pre>
</div>

<style lang="scss">
    .program {
        height: 50%;
    }
    .code {
        background-color: rgb(25, 25, 25);
        padding: 1em;
        border-radius: 8px;
        overflow-x: auto;
        overflow-y: auto;
        max-height: 70%;
    }
</style>
