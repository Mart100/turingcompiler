<script lang="ts">
  import Assembly from "./components/Assembly.svelte";
  import Controls from "./components/Controls.svelte";
  import Counter from "./components/Controls.svelte";
  import Instructions from "./components/Instructions.svelte";
  import Tape from "./components/Tape.svelte";
  import { interpreter, TuringMachine } from "./utils/Interpreter";

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
  }

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    // Handle the dropped files
    const files = event.dataTransfer?.files;
    if (files) {
      const file = files[0];
      const reader = new FileReader();
      reader.onload = function (e) {
        const contents = e.target!.result;
        interpreter.load_file(contents as string);
        console.log(interpreter);
      };
      reader.readAsText(file);
    }
  }

  function handleFileUpload(event: Event) {
    const target = event.target as HTMLInputElement;
    const file = target.files![0];
    const reader = new FileReader();
    reader.onload = function (e) {
      const contents = e.target!.result;
      interpreter.load_file(contents as string);
    };
    reader.readAsText(file);
  }

  let ready = false;

  interpreter.ready.subscribe((value) => {
    ready = value;
  });
</script>

<main on:dragover={handleDragOver} on:drop={handleDrop}>
  {#if !ready}
    <div class="dropzone">
      <h1>Drag and drop your txt file here</h1>
    </div>
    <label for="fileUpload"
      >Or click here: <input
        type="file"
        id="fileUpload"
        on:change={handleFileUpload}
      /></label
    >
  {:else}
    <Tape />
    <Controls />
    <div class="things">
      <Assembly />
      <Instructions />
    </div>
  {/if}
</main>

<style lang="scss">
  main {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
  }

  .things {
    width: 80%;
    display: flex;
    flex-direction: row;
    font-weight: bold;
    max-height: 60%;
    padding: 2em;
    background-color: rgb(30, 30, 30);
  }

  .dropzone {
    font-size: 0.7em;
  }
</style>
