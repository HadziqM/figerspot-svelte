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
  interface StartStop{
    start:string
    stop:string
  }

  const host = "http://127.0.0.1"
  const port = 8090


  let machine_items:Items[] = [];
  let inspected = false
  let start:string
  let stop:string

  const machineItems = async () =>{
    const [idk2,date2] = await Promise.all
    ([invoke("get_machine",{host,port}) as Promise<string>
      ,invoke("get_range",{host,port}) as Promise<string>])
    const idk = JSON.parse(idk2) as MachineItems
    const date = JSON.parse(date2) as StartStop

    start = date.start
    stop = date.stop
    machine_items = idk.items?idk.items:[]
    inspected = true
  }

  let greetMsg = ""
  let please_select:string
 
  async function greet(){
    const file = await save({filters: [{extensions: ["csv"],name: "data"}]});
    if (file==null){
      greetMsg = "No file selected"
    }else{
      greetMsg = "loading....."
      greetMsg = await invoke("get_all",{host,port,path:file,start,stop,machine:please_select}) as string
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
  <div class="nice-col">
    <select bind:value={please_select} on:change={()=>greetMsg=please_select}>
      {#each machine_items as item}
        <option value={item.id}>{item.name}</option>
      {/each}
    </select>
    <div class="nice-row">
    <p>Start</p>
    <input type="date" bind:value={start}>
    </div>
    <div class="nice-row">
    <p>Stop</p>
    <input type="date" bind:value={stop}>
    </div>
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
  .nice-col{
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  .nice-row{
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>
