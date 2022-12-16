<script lang="ts">
  import { open } from "@tauri-apps/api/dialog"
  import { invoke } from "@tauri-apps/api/tauri"
  const host = "http://127.0.0.1"
  const port = 8090

  let greetMsg = ""
  //async function greet(){
  //  const file = await open({filters:[{extensions:['csv'],name:"data"}]})
  //  if (file instanceof Array || file == null){
   //   greetMsg = "No file selected"
   // }else{ 
   //   greetMsg = await invoke("parse",{host,port,path:file}) as string
   // }
  //}
  let d_s = "01:00"
  let d_f ="02:00"
  function greet(){
    greetMsg = d_f
  }
  </script>

<div class="full_sc">
  <p>{greetMsg}</p>
  <form class="form-s" on:submit|preventDefault={greet}>
    <div class="bg-container">
      <div class="container">
        <p>Duhur</p>
        <div class="f-row">
        <div class="f-col"><label for="d_s">Start</label>
        <input type=time name="d_s" bind:value={d_s} ></div>
        <div class="f-col"><label for="d_f">Stop</label>
        <input type=time name="d_f" bind:value={d_f}></div>
        </div>
      </div>
      <div class="container">
        <p>Asyar</p>
        <div class="f-row">
        <div class="f-col"><label for="d_s">Start</label>
        <input type=time name="d_s" bind:value={d_s} ></div>
        <div class="f-col"><label for="d_f">Stop</label>
        <input type=time name="d_f" bind:value={d_f}></div>
        </div>
      </div>
      <div class="container">
        <p>Maghrib</p>
        <div class="f-row">
        <div class="f-col"><label for="d_s">Start</label>
        <input type=time name="d_s" bind:value={d_s} ></div>
        <div class="f-col"><label for="d_f">Stop</label>
        <input type=time name="d_f" bind:value={d_f}></div>
        </div>
      </div>
      <div class="container">
        <p>Isya'</p>
        <div class="f-row">
        <div class="f-col"><label for="d_s">Start</label>
        <input type=time name="d_s" bind:value={d_s} ></div>
        <div class="f-col"><label for="d_f">Stop</label>
        <input type=time name="d_f" bind:value={d_f}></div>
        </div>
      </div> 
      <div class="container">
        <p>Subuh</p>
        <div class="f-row">
        <div class="f-col"><label for="d_s">Start</label>
        <input type=time name="d_s" bind:value={d_s} ></div>
        <div class="f-col"><label for="d_f">Stop</label>
        <input type=time name="d_f" bind:value={d_f}></div>
        </div>
      </div>
      <div class="container">
        <p>Tahajud</p>
        <div class="f-row">
        <div class="f-col"><label for="d_s">Start</label>
        <input type=time name="d_s" bind:value={d_s} ></div>
        <div class="f-col"><label for="d_f">Stop</label>
        <input type=time name="d_f" bind:value={d_f}></div>
        </div>
      </div> 
    </div>
    <button type=submit >
      Proses Data
    </button>
  </form>
</div>

<style>
  .form-s{
    display: flex;
    flex-direction: column;
    padding: 1rem;
    border: solid #eee;
    border-radius: 10%;
  }
  .bg-container{
    display: grid;
    grid-template-columns: auto auto;
    padding: 0.5rem;
    gap: 0.3rem;
  }
  .container{
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    border: solid #aaa;
    border-radius: 10%;
    padding: 0.5rem;
  }
  .container p{
    width: 100%;
    background-color: #999;
    color: #333;
    text-align: center;
    font-size: 1rem;
    font-weight: bold;
  }
  .container label{
    width: 100%;
    text-align: center;
    font-size: 0.7rem;
  }
  .f-row{
    display: flex;
    gap: 1rem;
  }
  .f-col{
    display: flex;
    gap: 0.1rem;
    flex-direction: column;
  }

</style>
