<script lang="ts">
    import { interpreter } from "../utils/Interpreter";

    let error = "";

    interpreter.error.subscribe((value) => {
        error = value;
    });
</script>

{#if error.length > 0}
    <div class="error">ERROR: {error}</div>
{/if}

<style lang="scss">
    .error {
        color: red;
        font-size: 1.5em;
        font-weight: bold;
    }
</style>
