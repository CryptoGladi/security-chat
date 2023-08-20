<script lang="ts">
	import { onMount } from 'svelte';
	import Runner from './runner.svelte';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { invoke } from '@tauri-apps/api/tauri';
	import LoadingCenter from '$lib/loading_center.svelte';
	import { goto } from '$app/navigation';

	let runner_modal: HTMLDialogElement;

	onMount(async () => {
		await unregisterAll();

		await register('Ctrl+F', () => {
			runner_modal.showModal();
		});
	});

	async function getAllUsers() {
		return (await invoke('get_all_users')) as string[];
	}

	function openChat(nickname: string) {
		goto(`/chat/${nickname}`);
	}
</script>

{#await getAllUsers()}
	<LoadingCenter />
{:then users}
	{#each users as user}
		<div class="px-1 py-1 flex w-full" on:click={() => openChat(user)}>
			<div class="placeholder avatar">
				<div class="bg-primary rounded-full w-24 avatar">
					<span class="text-6xl text">{user[0].toUpperCase()}</span>
				</div>
			</div>

			<div class="mx-4 flex flex-col w-full h-full">
				<div class=" items-start">
					<strong>{user}</strong>
				</div>
			</div>
		</div>
	{/each}
{/await}

<dialog bind:this={runner_modal} class="modal">
	<Runner />
</dialog>
