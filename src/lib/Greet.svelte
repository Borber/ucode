<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri"

    let name = "";
    let greetMsg = ""

    async function greet() {
        greetMsg = await invoke("greet", {name})
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