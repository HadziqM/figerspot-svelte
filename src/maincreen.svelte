<script lang="ts">
  import Greet from "./lib/Greet.svelte";
  import Inspect from "./lib/Inspect.svelte";
</script>

  <div class="black">
  <Greet/>
  <Inspect/>
  </div>

<style>
  .black{
    display:flex;
    width:100vw;
    height: 100vh;
    justify-content: center;
    align-items: center;
  }
</style>
