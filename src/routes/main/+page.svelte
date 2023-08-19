<script lang="ts">
	import { onMount } from 'svelte';
	import Runner from './runner.svelte';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { invoke } from '@tauri-apps/api/tauri';
    import LoadingCenter from '$lib/loading_center.svelte';

	let runner_modal: HTMLDialogElement;

	onMount(async () => {
		await unregisterAll();

		await register('Ctrl+F', () => {
			runner_modal.showModal();
		});
	});

    async function getAllUsers() {
        return await invoke("get_all_users");
    }
</script>

{#await getAllUsers()}
<LoadingCenter></LoadingCenter>
{:then users}

<div class="px-1 py-1">
{#each users as user}
<div class="avatar placeholder">
    <div class="bg-neutral-focus text-neutral-content rounded-full w-24">
        <span class="text-3xl">{user[0]}</span>
    </div>
</div> 

{/each}
</div>
{/await}

<dialog bind:this={runner_modal} class="modal">
	<Runner />
</dialog>
