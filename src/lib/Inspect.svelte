<script lang="ts">
  import { save } from "@tauri-apps/api/dialog"
  import { invoke } from "@tauri-apps/api/tauri"
  const host = "http://127.0.0.1"
  const port = 8090

  let greetMsg = ""
  async function greet(){
    const file = await save({filters: [{extensions: ["csv"],name: "data"}]});
    if (file==null){
      greetMsg = "No file selected"
    }else{ 
      greetMsg = await invoke("get_all",{host:host,port:port,path:file})
    }
  }
</script>

<div>
  <div class="row">
    <button on:click={greet}>
      Get Data
    </button>
  </div>
  <p>{greetMsg}</p>
</div>
