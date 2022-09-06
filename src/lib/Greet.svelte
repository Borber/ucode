<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"

    let name = "";
    let greetMsg = ""

    async function greet() {
        let s = await invoke("lans");
        console.log(s[1]);
        greetMsg = await invoke("lans");
    }
</script>

<div>
    <div class="row">
        <input bind:value={name} id="greet-input" placeholder="Enter a name..."/>
        <button on:click={greet}>
            Greet
        </button>
    </div>
    <p>{greetMsg}</p>
</div>