<script lang="ts">
  import { interpreter } from "../utils/Interpreter";

  let output = "";
  let steps = 0;
  let state = "";
  let instructionCount = 0;

  interpreter.steps.subscribe((value) => {
    steps = value;
  });

  interpreter.state.subscribe((value) => {
    state = value;
  });

  interpreter.instructions.subscribe((value) => {
    instructionCount = value.size;
  });

  interpreter.output.subscribe((value) => {
    output = value;
  });
</script>

<div class="stats">
  <h2>Statistics</h2>
  <ul>
    <li>Current Step: <span>{steps}</span></li>
    <li>Current State: <span>{state}</span></li>
    <li>Total Instructions: <span>{instructionCount}</span></li>
    <li>OUTPUT: <span>{output}</span></li>
  </ul>
</div>

<style lang="scss">
  .stats {
    ul {
      list-style: none;
      padding: 0;
      padding-left: 15px;
      font-weight: normal;

      span {
        font-weight: bold;
      }
    }
  }
</style>
