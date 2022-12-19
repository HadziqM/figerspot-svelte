<script lang="ts">
  import { save } from "@tauri-apps/api/dialog"
  import { invoke } from "@tauri-apps/api/tauri"

  interface Items{
    id:string,
    name:string
  }

  interface MachineItems{
    items?:Items[]
  }

  const host = "http://127.0.0.1"
  const port = 8090


  let machine_items:Items[] = [];
  let inspected = false

  const machineItems = async () =>{
    const idk = JSON.parse(await invoke("get_machine",{host,port})) as MachineItems
    machine_items = idk.items?idk.items:[]
    inspected = true
  }

  let greetMsg = ""
  let please_select:string;

  async function greet(){
    const file = await save({filters: [{extensions: ["csv"],name: "data"}]});
    if (file==null){
      greetMsg = "No file selected"
    }else{
      greetMsg = "loading....."
      greetMsg = await invoke("get_all",{host:host,port:port,path:file,start:"2021-01-01",stop:"2022-12-12",machine:please_select}) as string
    }
    setTimeout(()=>greetMsg="",3000)
  }
</script>

<div class="full_sc">
  {#if !inspected}
  <button class="inspect" on:click={machineItems}>
    inspect
  </button>
  {:else}
  <p class="status">Please Select Machine</p>
  <div class="row">
    <select bind:value={please_select} on:change={()=>greetMsg=please_select}>
      {#each machine_items as item}
        <option value={item.id}>{item.name}</option>
      {/each}
    </select>
    <button on:click={greet}>
      Get Data
    </button>
  </div>
  {/if}
  <p class="status-idk">{greetMsg}</p>
</div>
<style>
  .inspect{
    font-size: 1.5rem;
    padding: 0.5rem;
  }
  .status{
    font-size: 1.5rem;
  }
  .status-idk{
    position: absolute;
    top: 20vh;
    font-size:1.5rem;
  }
</style>
