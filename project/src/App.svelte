<script lang="ts">
  import Compiler from "./components/Compiler.svelte";
  import Debugger from "./components/Debugger.svelte";
  import { interpreter } from "./utils/Interpreter";

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

  function codeCompiled(turing_program: string) {
    interpreter.load_file(turing_program);
  }
</script>

<main on:dragover={handleDragOver} on:drop={handleDrop}>
  <Compiler {codeCompiled} />
  {#if ready}
    <Debugger />
  {/if}
</main>

<style lang="scss">
  main {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    flex-direction: column;
  }
</style>
