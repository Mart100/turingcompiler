<script lang="ts">
    import {
        interpreter,
        type AssemblyInstruction,
        type TapeSection,
    } from "../utils/Interpreter";

    let asmInstructions: AssemblyInstruction[] = [];

    interpreter.assemblyInstructions.subscribe((value) => {
        asmInstructions = value;
    });

    let sections: TapeSection[] = [];

    interpreter.sections.subscribe((value) => {
        sections = value;
    });
</script>

<div class="assembly">
    {#if asmInstructions.length > 0 && asmInstructions[0].sections}
        <table class="assembly-instructions">
            <thead>
                <tr>
                    <th>Instruction</th>
                    {#each sections as section, index (index)}
                        <th>{section.name}</th>
                    {/each}
                </tr>
            </thead>
            <tbody>
                {#each asmInstructions as instruction, index (index)}
                    <tr
                        class="instruction-item"
                        style="background-color: {instruction.status ==
                        'current'
                            ? 'red'
                            : instruction.status == 'completed'
                              ? 'darkolivegreen'
                              : 'transparent'}"
                    >
                        <td class="instruction">{instruction.instruction}</td>
                        {#if instruction.sections}
                            {#each instruction.sections as section, index (index)}
                                <td>{section.value}</td>
                            {/each}
                        {/if}
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}
</div>

<style lang="scss">
    .assembly {
        text-align: left;

        .instruction-item {
            transition: background-color 0.25s;

            .instruction {
                padding-right: 2em;
            }
        }
    }

    :global(.sections) {
        margin-left: 2em;
        background-color: darkolivegreen;
        white-space: nowrap;
    }
</style>
