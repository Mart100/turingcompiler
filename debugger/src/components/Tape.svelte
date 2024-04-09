<script lang="ts">
    import { interpreter, type TapeSection } from "../utils/Interpreter";

    let head = 0;
    let tape: number[] = [];
    let sections: TapeSection[] = [];

    interpreter.head.subscribe((value) => {
        head = value;
    });

    interpreter.sections.subscribe((value) => {
        sections = value;
    });

    interpreter.tape.subscribe((value) => {
        tape = value;

        // if sections exist, update values.
        if (sections.length > 1) {
            sections.forEach((section, index) => {
                let values = "";
                let value = "";

                for (let i = section.start; i <= section.end; i++) {
                    values += tape[i];
                    if (tape[i] != 0 && tape[i] != 1) {
                        value = "x";
                    }
                }

                // transform binary values to decimal
                if (value != "x") value = parseInt(values, 2).toString();
                sections[index].value = value;
            });
        }
    });
</script>

<div class="tape">
    <div class="sections">
        {#each sections as section, index (index)}
            <div
                class="section-item"
                style="grid-column: {section.start + 1} / {section.end +
                    2}; background-color: {section.color};"
            >
                {section.name} = {section.value}
            </div>
        {/each}
    </div>
    <div class="items">
        {#each tape as item, index (index)}
            <div
                class="tape-item"
                style="
                grid-column: {index + 1}; 
                background-color: {index === head ? 'darkturquoise' : 'black'};
            "
            >
                {item}
            </div>
        {/each}
    </div>
</div>

<style lang="scss">
    .tape {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(20px, 1fr));
        width: 80%;
        overflow-x: auto;
        grid-auto-flow: row;
        grid-template-rows: auto auto;
        font-weight: bold;

        .sections {
            display: contents;
            .section-item {
                display: inline-block;
                padding: 0.2rem;
            }
        }

        .items {
            display: contents;
            .tape-item {
                display: inline-block;
                width: 20px;
                border-left: 1px solid white;
                background-color: black;
            }
        }
    }
</style>
