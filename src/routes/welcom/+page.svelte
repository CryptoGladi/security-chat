<script lang="ts">
	import RegistrationModal from './registration_modal.svelte';
	import VscGithubInverted from 'svelte-icons-pack/vsc/VscGithubInverted';
	import Icon from 'svelte-icons-pack/Icon.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let registration_modal: HTMLDialogElement;
</script>

<div class="text-center hero min-h-screen">
	<div class="max-w-md">
		<h1 class="text-5xl font-bold">Добро пожаловать</h1>
		<div class="py-6">
			<p>
				<strong>Security chat</strong> - это универсальный чат который даёт вам приватность и анонимность
			</p>
			<p>Для начала использования программы вам завершить её настройку</p>
		</div>

		<button
			class="btn btn-primary"
			on:click={() => {
				registration_modal.showModal();
			}}>Начать</button
		>

		<dialog bind:this={registration_modal} class="modal">
			<RegistrationModal />
		</dialog>
	</div>
</div>

<div class="bottom-0 right-0 flex space-x-2 px-1 py-1 w-10" style="position: absolute;">
	<div
		class="w-full h-full"
		on:click={() => invoke('open', { path: 'https://github.com/CryptoGladi/security-chat' })}
	>
		<Icon src={VscGithubInverted} className="w-full h-full" color="white" />
	</div>
</div>

<div class="left-0 top-0 flex space-x-2 px-1 py-1" style="position: absolute;">
	<div class="badge badge-secondary">Версия 0.1</div>
</div>