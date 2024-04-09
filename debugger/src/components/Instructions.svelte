<script lang="ts">
    import { interpreter, type TuringInstruction } from "../utils/Interpreter";

    let instructionsArray: [string, TuringInstruction][] = [];
    let currentSection = 0;
    let currentInstruction = "";

    interpreter.currentInstruction.subscribe((value) => {
        currentInstruction = value;
    });

    interpreter.instructions.subscribe((value) => {
        instructionsArray = Array.from(value);
    });

    interpreter.currentAsmInstruction.subscribe((value) => {
        currentSection = value;
    });

    let activeRow: HTMLElement | null = null;
    let rowElements: HTMLElement[] = [];

    // Update rowElement in a reactive statement
    $: {
        for (let index in instructionsArray) {
            let instruction = instructionsArray[index];
            if (instruction[0] == currentInstruction) {
                activeRow = rowElements[index];
                break;
            }
        }
    }

    $: if (activeRow) {
        activeRow.scrollIntoView({ behavior: "instant", block: "nearest" });
    }
</script>

<div class="table-container">
    <table class="table">
        <thead>
            <tr>
                <th>Instruction</th>
                <th>Read</th>
                <th>Write</th>
                <th>Action</th>
                <th>Next State</th>
            </tr>
        </thead>
        <tbody class="instructions">
            {#each instructionsArray as instruction, index}
                {#if instruction[1].asm == currentSection}
                    <tr
                        bind:this={rowElements[index]}
                        class:active={instruction[0] == currentInstruction}
                    >
                        <td>{instruction[0].split("-")[0]}</td>
                        <td>{instruction[0].split("-")[1]}</td>
                        <td>{instruction[1].write}</td>
                        <td>{instruction[1].action}</td>
                        <td>{instruction[1].next_state}</td>
                    </tr>
                {/if}
            {/each}
        </tbody>
    </table>
</div>

<style lang="scss">
    .table-container {
        height: 100%;
        overflow-y: auto;
        overflow-x: hidden;
        margin-left: 4em;

        .table {
            text-align: left;
            list-style-type: none;
            height: 100%;
        }
        .active {
            background-color: red;
        }
    }
</style>
