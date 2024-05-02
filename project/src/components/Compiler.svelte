<script lang="ts">
  //@ts-ignore
  import init, { compile, lexer, type CompileResult } from "webcompiler";
  init();

  import { getCodeExamples } from "../utils/Examples";
  import CodeEditor from "./CodeEditor.svelte";

  type Error = {
    error: string;
  };

  export let codeCompiled = (turing_program: string) => {};

  let getCode: () => string;
  let setCode: (code: string) => void;
  let result: CompileResult | Error | null = null;

  // Hijack errors
  let lastError = "";
  if (window.console && console.error) {
    let ce = console.error;
    console.error = function (error: string) {
      lastError = error;
      ce.apply(this, arguments as any);
    };
  }

  function compileCode() {
    let code = getCode();
    try {
      result = compile(code);
    } catch (e: any) {
      let err = lastError;

      // remove everything after "Stack:"
      let stackIndex = err.indexOf("Stack:");
      if (stackIndex !== -1) {
        err = err.substring(0, stackIndex);
      }

      // Remove the first line
      let firstLineIndex = err.indexOf("\n");
      if (firstLineIndex !== -1) {
        err = err.substring(firstLineIndex + 1);
      }

      result = { error: err };
    }
    console.log(result);
    if (result !== null && !("error" in result)) {
      codeCompiled(result.turing_program);
    }
  }

  let activeTab = "Tokens";
  const tabs = [
    "Tokens",
    "AST",
    "TAC",
    "Optimized TAC",
    "Assembly",
    "Turing Instructions",
  ];

  let exampleButtonsExpanded = false;
  const examples = getCodeExamples();
  const exampleNames = Object.keys(examples);

  function loadExample(example: string) {
    setCode(examples[example as keyof typeof examples].trim());
    exampleButtonsExpanded = false;
  }
</script>

<div class="compiler">
  <div class="buttons">
    <div
      class="example-buttons"
      role="button"
      tabindex="0"
      on:mouseenter={() => {
        exampleButtonsExpanded = true;
      }}
      on:mouseleave={() => {
        exampleButtonsExpanded = false;
      }}
    >
      <button>Load Example</button>
      {#if exampleButtonsExpanded}
        <div class="drawer">
          {#each exampleNames as example, i}
            <button class="example-button" on:click={() => loadExample(example)}
              >{example}</button
            >
          {/each}
        </div>
      {/if}
    </div>
  </div>
  <div class="row">
    <CodeEditor {lexer} bind:getCode bind:setCode></CodeEditor>

    <div class="middle">
      <svg
        class="arrow"
        xmlns="http://www.w3.org/2000/svg"
        width="80"
        height="80"
        viewBox="0 0 24 24"
        ><path
          fill="currentColor"
          d="m14 18l-1.4-1.45L16.15 13H4v-2h12.15L12.6 7.45L14 6l6 6z"
        /></svg
      >
      <button on:click={compileCode}>Compile</button>
    </div>

    <div class="result">
      {#if result === null}
        <p>Compile your code to see the result</p>
      {:else if "error" in result}
        <pre class="error">{result.error}</pre>
      {:else if "turing_program" in result}
        <div class="tabs">
          {#each tabs as tab}
            <button
              class:active={activeTab === tab}
              on:click={() => (activeTab = tab)}>{tab}</button
            >
          {/each}
        </div>
        <div class="content">
          {#if activeTab === "Tokens"}
            <pre>{result.tokens}</pre>
          {:else if activeTab === "AST"}
            <pre>{result.ast}</pre>
          {:else if activeTab === "TAC"}
            <pre>{result.tac}</pre>
          {:else if activeTab === "Optimized TAC"}
            <pre>{result.optimized_tac}</pre>
          {:else if activeTab === "Assembly"}
            <pre>{result.assembly}</pre>
          {:else if activeTab === "Turing Instructions"}
            <pre>{result.turing_program}</pre>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
  .compiler {
    margin-top: 3%;
    .buttons {
      text-align: left;
      .example-buttons {
        position: relative;
        display: inline-block;
        margin-bottom: 20px;

        > button {
          cursor: default;
        }

        .drawer {
          position: absolute;
          z-index: 100;
        }

        .example-button {
          display: block;
          text-align: left;
          width: fit-content;
          text-wrap: nowrap;
          top: 0;
          left: 0;
          transition: transform 0.3s ease;
          border-radius: 0px;
        }
      }
    }
    .row {
      display: flex;
      height: 500px;
    }

    .middle {
      margin: auto 1rem;
      display: flex;
      flex-direction: column;
      align-items: center;
    }

    .result {
      display: flex;
      flex-direction: column;
      width: 400px;
      background-color: rgb(18, 18, 18);
      overflow-x: auto;

      .error {
        color: red;
        text-wrap: wrap;
      }

      .tabs {
        display: flex;
        flex-direction: row;
        gap: 0rem;
        align-items: center;

        button {
          font-size: 0.8rem;
          padding: 0.5rem;
          border: none;
          cursor: pointer;
          height: 100%;
          border-radius: 0px;
          outline: none;

          &:hover {
            background-color: rgb(22, 22, 22);
          }

          &.active {
            background-color: rgb(18, 18, 18);
          }
        }
      }

      .content {
        flex: 1;
        padding: 1rem;
        height: 100%;
        background-color: rgb(18, 18, 18);
        margin: 0px;
        text-align: left;
        overflow-y: auto;

        pre {
          margin: 0px;
        }
      }
    }
  }
</style>
