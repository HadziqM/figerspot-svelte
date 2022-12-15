<script lang="ts">
  import { open } from "@tauri-apps/api/dialog"
  import { invoke } from "@tauri-apps/api/tauri"
  const host = "http://127.0.0.1"
  const port = 8090

  let greetMsg = ""
  async function greet(){
    const file = await open({filters:[{extensions:['csv'],name:"data"}]})
    if (file instanceof Array || file == null){
      greetMsg = "No file selected"
    }else{ 
      greetMsg = await invoke("parse",{host,port,path:file})
    }
  }
  async function delete_all(){
    await invoke("remove",{host,port})
  }
</script>

<div>
  <div class="row">
    <button on:click={greet}>
      Proses Data
    </button>
    <button on:click={delete_all}>
      Delete
    </button>
  </div>
  <p>{greetMsg}</p>
</div>
