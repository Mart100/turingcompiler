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
        <th>R</th>
        <th>W</th>
        <th>A</th>
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
            <td data-tooltip={instruction[0].split("-")[0]}
              >{instruction[0].split("-")[0]}</td
            >
            <td>{instruction[0].split("-")[1]}</td>
            <td>{instruction[1].write}</td>
            <td>{instruction[1].action}</td>
            <td data-tooltip={instruction[1].next_state}
              >{instruction[1].next_state}</td
            >
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
    max-width: 30%;

    .table {
      text-align: left;
      list-style-type: none;

      table-layout: fixed;

      thead th {
        position: sticky;
        top: 0;
        background: black;
        z-index: 100;
      }

      tbody tr td:first-child,
      td:last-child {
        max-width: 140px;
        overflow: hidden;
        text-overflow: ellipsis;
        position: relative;
        z-index: 0;

        &:hover {
          overflow: visible;
          text-overflow: ellipsis;
        }

        &:hover::after {
          content: attr(data-tooltip);
          position: absolute;
          left: 0;
          top: 0;
          white-space: nowrap;
          background-color: black;
          color: #fff;
          border-radius: 5px;
          z-index: 100;
          overflow: visible;
        }
      }

      tbody tr td:last-child {
        &:hover::after {
          left: auto;
          right: 0;
          min-width: 140px;
        }
      }
    }
    .active {
      background-color: red;
    }
  }
</style>
