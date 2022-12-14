<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import {open} from "@tauri-apps/api/dialog"

  let name = "";
  let greetMsg = ""
  
  async function greet(){
    const file = await open({filters:[{extensions:['csv'],name:"data"}]})
    greetMsg = await invoke("testing", { path:file })
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>
