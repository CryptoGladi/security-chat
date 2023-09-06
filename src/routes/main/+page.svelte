<script lang="ts">
	import { onMount } from 'svelte';
	import Runner from './runner.svelte';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { invoke } from '@tauri-apps/api/tauri';
	import LoadingCenter from '$lib/loading_center.svelte';
	import { goto } from '$app/navigation';
	import { listen } from '@tauri-apps/api/event';
	import Icon from 'svelte-icons-pack';
	import FaSolidUserFriends from 'svelte-icons-pack/fa/FaSolidUserFriends';

	let runner_modal: HTMLDialogElement;

	listen('new-accept-aes-key', (_event) => {
		goto('/main');
	});

	onMount(async () => {
		await unregisterAll();
		// TODO not global!
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

<div class="navbar bg-neutral text-neutral-content flex-initial">
	<div class="flex-1 space-x-2">
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<a
			class="btn btn-ghost normal-case text-xl"
			on:click={() => {
				runner_modal.showModal();
			}}>Выполнить...</a
		>

		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div on:click={() => goto('/add_crypto')}>
			<Icon src={FaSolidUserFriends} size="40px" />
		</div>
	</div>

	<div class="flex-none gap-2">
		{#await invoke('get_nickname') then nickname}
			<span class="text-3xl text">{nickname}</span>
		{/await}
	</div>
</div>

{#await getAllUsers()}
	<LoadingCenter />
{:then users}
	{#each users as user}
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<!-- svelte-ignore a11y-no-static-element-interactions -->
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
