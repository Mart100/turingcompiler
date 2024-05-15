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

  const breakpointClick = (event: MouseEvent) => {
    const target = event.target as HTMLElement;
    const brIdx = target.id.split("-")[1];
    interpreter.set_breakpoint(Number(brIdx));
  };
</script>

<div class="assembly">
  {#if asmInstructions.length > 0 && asmInstructions[0].sections}
    <table>
      <thead>
        <tr>
          <th>Br</th>
          <th>Instruction</th>
          {#each sections as section, index (index)}
            <th>{section.name}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each asmInstructions as instruction, index (index)}
          <tr class="instruction-item">
            <td
              class="breakpoint"
              class:active={instruction.breakpoint}
              id="breakpoint-{index}"
              on:click={breakpointClick}
            ></td>
            <td
              class="instruction"
              style="background-color: {instruction.status == 'current'
                ? 'red'
                : instruction.status == 'completed'
                  ? 'darkolivegreen'
                  : 'transparent'}">{instruction.instruction}</td
            >
            {#if instruction.sections && instruction.sections.length > 0}
              {#each instruction.sections as section, index (index)}
                <td class="section">{section.value}</td>
              {/each}
            {:else}
              <td>-</td>
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
    overflow-y: auto;
    display: flex;
    overflow-x: auto;
    max-width: 40%;

    thead th {
      position: sticky;
      top: 0;
      background: black;
      z-index: 100;
    }

    .instruction-item {
      transition: background-color 0.25s;
      white-space: nowrap;

      .breakpoint {
        &:hover::after {
          content: "";
          display: inline-block;
          width: 10px;
          height: 10px;
          background-color: lightcoral;
          border-radius: 50%;
        }
        &.active::after {
          content: "";
          display: inline-block;
          width: 10px;
          height: 10px;
          border-radius: 50%;
          background-color: red;
        }
      }

      td:nth-child(2n + 3) {
        background-color: rgb(60, 60, 60);
      }
    }
  }
</style>
