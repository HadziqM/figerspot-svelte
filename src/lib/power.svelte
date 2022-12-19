<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  const host = "http://127.0.0.1"
  const port = 8090

  let greetMsg = ""
  async function greet(){
    greetMsg= "loading....."
    greetMsg = await invoke("remove",{host:host,port:port}) as string
    setTimeout(()=>greetMsg="",3000)
  }
</script>

<div class="full_sc">
  <div class="delete">
  <p>{greetMsg}</p>
    <button on:click={greet}>
      Delete All Database Data
    </button>
  </div>
</div>

<style>
  .delete{
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 1rem;
    font-size: 1.3rem;
    font-weight: 600;
  }
  .delete button{
    background-color: red;
    color: white;
    padding: 1rem;
  }
</style>
