<script lang="ts">
  import Controls from "./Controls.svelte";
  import Tape from "./Tape.svelte";
  import Error from "./Error.svelte";
  import Assembly from "./Assembly.svelte";
  import Instructions from "./Instructions.svelte";
  import Program from "./Program.svelte";
  import Statistics from "./Statistics.svelte";
</script>

<div class="debugger">
  <Tape />
  <Controls />
  <Error />
  <div class="things">
    <Assembly />
    <Instructions />
    <div class="things2">
      <Program />
      <Statistics />
    </div>
  </div>
</div>

<style lang="scss">
  .debugger {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    margin-top: 3%;
    height: 100vh;
    width: 100%;
  }

  .things {
    width: 80%;
    display: flex;
    flex-direction: row;
    font-weight: bold;
    height: 60%;
    padding: 2em;
    background-color: rgb(30, 30, 30);
    justify-content: space-between;

    .things2 {
      text-align: left;
      width: 25%;
    }
  }
</style>
