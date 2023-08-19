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
        return await invoke("get_all_users") as string[];
    }

	function openChat(nickname: string) {
		console.log(1111);
	}
</script>

{#await getAllUsers()}
<LoadingCenter></LoadingCenter>
{:then users}
{#each users as user}

<div class="px-1 py-1 flex" on:click={() => openChat(user)}>
<div class="placeholder avatar">
    <div class="bg-primary rounded-full w-24 avatar">
        <span class="text-6xl text">{user[0].toUpperCase()}</span>
    </div>
</div> 

<div class="px-5">
	<strong>{user}</strong>

	<div class="self-end my-3">
		<p class="">Последнее сообщение</p>
	</div>
</div>
</div>

{/each}
{/await}

<dialog bind:this={runner_modal} class="modal">
	<Runner />
</dialog>
