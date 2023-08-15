<script>
  import "../TailwindCSS.css";
  import "material-icons/iconfont/material-icons.css";
  import { register } from "@tauri-apps/api/globalShortcut";
  import { fuzzy_search_command, run_command, get_all_users } from "../Api"

  let isSearchOpen = false;

  register("Ctrl+F", () => {
    isSearchOpen = true;
  });

  let commandGetFuzzy;
  let commandsGetFuzzyReturn = [];
  function getFuzzy() {
    fuzzy_search_command(commandGetFuzzy).then((commands) => {
      commandsGetFuzzyReturn = commands;
    });
  }

  let promise_users = get_all_users();
</script>

{#await promise_users}
	<p>...waiting</p>
{:then users}
{#each users as user }
<div class="avatar placeholder flex py-1 px-1">
  <div class="bg-nord10 text-white rounded-full w-20">
    <span class="text-3xl">G</span>
  </div>
  <p>{user}</p>
  <p class="text-white self-end py-3 px-2">Вы: Моё после сообщение</p>
</div>
{/each}
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}

<div class="bg-nord1 absolute inset-x-0 bottom-0 h-6 flex">
  <span
    on:click={() => (isSearchOpen = true)}
    class="material-icons text-black bg-nord10 absolute">search</span
  >
</div>

<input
  type="checkbox"
  id="my-modal"
  class="modal-toggle"
  bind:checked={isSearchOpen}
/>

<!-- svelte-ignore a11y-autofocus -->
<div
  class="modal shadow-lg bg-transparent"
  on:click|self={() => (isSearchOpen = false)}
>
  <div class="w-[60%] bg-nord1 py-2 px-2 rounded-lg">
    <input
      type="text"
      placeholder="Ваша команда?"
      class="input input-bordered w-full text-slate-300"
      bind:value={commandGetFuzzy}
      on:input={getFuzzy}
      on:keydown={(e) => {
        if (e.code == "Enter") {
          run_command(commandGetFuzzy);
        }
      }}
    />

    {#each commandsGetFuzzyReturn as i}
      <p class="text-slate-400">{i}</p>
    {/each}
  </div>
</div>