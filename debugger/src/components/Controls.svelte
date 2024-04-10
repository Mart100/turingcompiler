<script lang="ts">
  import { interpreter } from "../utils/Interpreter";

  let status: string = "paused";

  interpreter.status.subscribe((value) => {
    status = value;
  });

  const changeStatus = () => {
    if (status == "paused") {
      interpreter.run();
    } else if (status == "running") {
      interpreter.pause();
    }
  };

  const reset = () => {
    status = "paused";
    interpreter.pause();
    interpreter.reset();
  };

  const step = () => {
    interpreter.step();
  };

  let speed = "10";

  interpreter.speed.subscribe((value) => {
    speed = value.toString();
  });

  const changeSpeed = (event: Event) => {
    interpreter.setSpeed(Number((event.target as HTMLSelectElement).value));
  };

  const removeProgram = () => {
    interpreter.delete();
  };
</script>

<div class="controls">
  <button on:click={changeStatus}
    >{status == "paused" ? "Start" : "Pause"}</button
  >
  <button on:click={reset}>Reset</button>
  <button on:click={step}>Step</button>
  <select bind:value={speed} on:change={changeSpeed}>
    <!-- Add this dropdown -->
    <option value="1">Very Slow</option>
    <option value="5">Slow</option>
    <option value="10">Average</option>
    <option value="100">Fast</option>
    <option value="500">Very Fast</option>
    <option value="999">Super Fast</option>
    <option value="1000">Full Speed</option>
  </select>
  <button on:click={removeProgram}>Remove Program</button>
</div>

<style lang="scss">
  .controls {
    margin: 2em 0;
  }

  select {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    background-color: #1a1a1a;
    cursor: pointer;
    transition: border-color 0.25s;
  }

  select:hover {
    border-color: #646cff;
  }
</style>
