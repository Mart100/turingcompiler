<script lang="ts">
  import { onMount } from "svelte";

  export let lexer: (code: string) => string;
  let lineNumbers = "1";
  let highlightedCode = "";
  let codeElement: HTMLTextAreaElement | null = null;
  const tokenColors = {
    operator: "#d19a66",
    keyword: "#c586c0",
    identifier: "#9cdcfe",
    parenthesis: "#569cd6",
    number: "#b5cea8",
    semicolon: "#d4d4d4",
    comparisonoperator: "#d19a66",
    bracket: "#d4d4d4",
  };

  onMount(() => {
    codeChanged();
    fixTextAreaHeight();
  });

  export function setCode(value: string) {
    if (codeElement) {
      codeElement.value = value;
      codeChanged();
    }
  }

  export function getCode() {
    return codeElement!.value;
  }

  function codeChanged() {
    let code = codeElement!.value;
    let tokensJson = lexer(code);
    let tokens = JSON.parse(tokensJson) as { type_: string; value: string }[];

    highlightedCode = "";
    let codeIndex = 0;

    for (const token of tokens) {
      let tokenIndex = code.indexOf(token.value, codeIndex);
      if (tokenIndex !== -1) {
        // Add the code before the token, preserving the original formatting
        highlightedCode += code.substring(codeIndex, tokenIndex);

        let color =
          tokenColors[token.type_.toLowerCase() as keyof typeof tokenColors];

        // Add the token with the highlighting
        highlightedCode += `<span style="color: ${color}">${token.value}</span>`;

        // Move the code index after the token
        codeIndex = tokenIndex + token.value.length;
      }
    }

    // Add the remaining code after the last token, preserving the original formatting
    highlightedCode += code.substring(codeIndex);

    fixTextAreaHeight();
    updateLineNumbers();
  }

  function updateLineNumbers() {
    let lines = getCode().split("\n");
    lineNumbers = lines.map((_, i) => i + 1).join("\n");
  }

  function fixTextAreaHeight() {
    if (codeElement) {
      codeElement.style.height = "1px";
      codeElement.style.height = codeElement.scrollHeight + "px";
      console.log(codeElement.style.height);
    }
  }

  function keydown(event: KeyboardEvent) {
    if (event.key === "Tab" && codeElement) {
      let start = codeElement.selectionStart;
      let end = codeElement.selectionEnd;
      let value = getCode();

      setCode(value.substring(0, start) + "    " + value.substring(end));

      codeElement.selectionStart = codeElement.selectionEnd = start + 4;

      event.preventDefault();
    }
  }
</script>

<div class="code-editor">
  <div class="line-numbers">
    <pre>{lineNumbers}</pre>
  </div>
  <textarea
    bind:this={codeElement}
    on:input={codeChanged}
    on:keydown={keydown}
    placeholder="Enter your code here..."
  ></textarea>
  <pre>{@html highlightedCode}</pre>
</div>

<style lang="scss">
  .code-editor {
    width: 400px;
    position: relative;
    overflow-y: auto;
    background-color: rgb(18, 18, 18);

    .line-numbers {
      position: absolute;
      top: 0;
      left: 0;
      width: 20px;
      color: rgb(165, 165, 165);
      text-align: right;
      padding-right: 5px;
      padding-top: 1rem;

      > pre {
        margin-top: 0px;
        font-size: 1rem;
        line-height: normal;
      }
    }

    > textarea {
      box-sizing: border-box;
      width: 100%;
      height: 51px;
      padding: 1rem;
      font-size: 1rem;
      font-family: monospace;
      border: none;
      outline: none;
      resize: none;
      color: transparent;
      caret-color: white;
      padding-left: calc(1rem + 20px);
    }

    > pre {
      box-sizing: border-box;
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      font-size: 1rem;
      font-family: monospace;
      pointer-events: none;
      text-align: left;
      padding: 1rem;
      margin: 0px;
      line-height: normal;
      padding-left: calc(1rem + 20px);
    }
  }
</style>
