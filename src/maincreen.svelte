<script lang="ts">
  import Greet from "./lib/Greet.svelte";
  import Inspect from "./lib/Inspect.svelte";
  import Power from "./lib/power.svelte";

  let selected = 'time'
</script>

  <div class="black">
  <div class="side-bar">
    <div 
      class="{selected==='time'?'selected':''}" 
      on:click="{()=>selected='time'}">
     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. --><path d="M152 64H296V24C296 10.75 306.7 0 320 0C333.3 0 344 10.75 344 24V64H384C419.3 64 448 92.65 448 128V448C448 483.3 419.3 512 384 512H64C28.65 512 0 483.3 0 448V128C0 92.65 28.65 64 64 64H104V24C104 10.75 114.7 0 128 0C141.3 0 152 10.75 152 24V64zM48 248H128V192H48V248zM48 296V360H128V296H48zM176 296V360H272V296H176zM320 296V360H400V296H320zM400 192H320V248H400V192zM400 408H320V464H384C392.8 464 400 456.8 400 448V408zM272 408H176V464H272V408zM128 408H48V448C48 456.8 55.16 464 64 464H128V408zM272 192H176V248H272V192z"/></svg>
    </div
    >
    <div
      class="{selected==='doc'?'selected':''}" 
      on:click="{()=>selected='doc'}"
      >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><!--! Font Awesome Pro 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. --><path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V285.7l-86.8 86.8c-10.3 10.3-17.5 23.1-21 37.2l-18.7 74.9c-2.3 9.2-1.8 18.8 1.3 27.5H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128zM549.8 235.7l14.4 14.4c15.6 15.6 15.6 40.9 0 56.6l-29.4 29.4-71-71 29.4-29.4c15.6-15.6 40.9-15.6 56.6 0zM311.9 417L441.1 287.8l71 71L382.9 487.9c-4.1 4.1-9.2 7-14.9 8.4l-60.1 15c-5.5 1.4-11.2-.2-15.2-4.2s-5.6-9.7-4.2-15.2l15-60.1c1.4-5.6 4.3-10.8 8.4-14.9z"/></svg>
    </div>
    <div
      class="{selected==='pow'?'selected':''}" 
      on:click="{()=>selected='pow'}"
      >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--! Font Awesome Pro 6.2.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. --><path d="M288 32c0-17.7-14.3-32-32-32s-32 14.3-32 32V256c0 17.7 14.3 32 32 32s32-14.3 32-32V32zM143.5 120.6c13.6-11.3 15.4-31.5 4.1-45.1s-31.5-15.4-45.1-4.1C49.7 115.4 16 181.8 16 256c0 132.5 107.5 240 240 240s240-107.5 240-240c0-74.2-33.8-140.6-86.6-184.6c-13.6-11.3-33.8-9.4-45.1 4.1s-9.4 33.8 4.1 45.1c38.9 32.3 63.5 81 63.5 135.4c0 97.2-78.8 176-176 176s-176-78.8-176-176c0-54.4 24.7-103.1 63.5-135.4z"/></svg>
    </div>
  </div>
  {#if selected==='time'}
    <Greet/>
  {:else}
    {#if selected==='doc'}
      <Inspect/>
    {:else}
      <Power/>
    {/if}
  {/if}
  </div>

<style>
  .black{
    position: relative;
    display:flex;
    width:100vw;
    height: 100vh;
    justify-content: center;
    align-items: center;
  }
  .side-bar{
    position: absolute;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    left: 0;
    height: 100vh;
    width: 10vw;
    background-color: rgba(0, 0, 0, 0.5);
  }
  .side-bar svg{
    width: 3rem;
    height: 3rem;
    color: #aaa;
  }
  .side-bar div{
    display: flex;
    justify-content: center;
    justify-items: center;
    width: 100%;
    cursor: pointer;
    padding: 1rem 0;
  }
  .selected{
    background-color: rgba(255, 255, 255, 0.6);
  }
</style>
