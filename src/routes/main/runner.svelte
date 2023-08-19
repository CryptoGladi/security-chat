<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

    let fuzyyCommands: string[] = [];

    function getFuzzy(command: string) {
        invoke("fuzzy_search_vim_command", { command: command }).then((commands) => {
            fuzyyCommands = commands as string[];
        });
  }
</script>

<form method="dialog" class="modal-box">
    <input type="text" placeholder="Ваша команда" on:input={e => getFuzzy(e.target.value)} class="input input-bordered input-accent w-full" />

    <div class="mt-3">
        {#each fuzyyCommands as command}
        <p>{command}</p>
        {/each}
    </div>
</form>
<form method="dialog" class="modal-backdrop">
	<button>close</button>
</form>
