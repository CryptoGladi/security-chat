<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import isAlphanumeric from 'validator/lib/isAlphanumeric';
	import { notifier } from '@beyonk/svelte-notifications';
	import { goto } from '$app/navigation';

	enum StateNickname {
		Empty,
		IsTaken,
		IsFree
	}

	let state_nickname = StateNickname.Empty;
	let nickname = '';
	let registration_button: HTMLButtonElement;

	function changeNickname(input_nickname: string) {
		if (input_nickname == '') {
			state_nickname = StateNickname.Empty;
			registration_button.disabled = true;
		} else {
			invoke('nickname_is_taken', { nickname: input_nickname }).then((nickname_is_taken) => {
				console.log(`nicname_is_taken: ${nickname_is_taken}`);
				switch (nickname_is_taken) {
					case true:
						state_nickname = StateNickname.IsTaken;
						registration_button.disabled = true;
						break;
					case false: {
						state_nickname = StateNickname.IsFree;
						registration_button.disabled = false;
						break;
					}
				}
			});
		}
	}

	function createAccout() {
		if (isAlphanumeric(nickname)) {
			invoke('registration', { nickname: nickname }).then(() => {
				goto('/main');
			});
		} else notifier.danger('Ошибка! У вас в нике запрещенные символы', 10000);
	}
</script>

<form method="dialog" class="modal-box">
	<h3 class="font-bold text-lg">Регистрация</h3>

	<div class="py-4">
		<input
			type="text"
			on:input={(e) => {
				// @ts-ignore
				changeNickname(e.target?.value);
			}}
			bind:value={nickname}
			placeholder="Ваш никнейм"
			class="input input-bordered w-full max-w-xs"
			on:keypress={(e) => {
				if (e.charCode == 32) e.preventDefault();
			}}
		/>

		{#if state_nickname == StateNickname.Empty}
			<p class="mt-2 text-sm text-accent">
				<strong>Ваш ник должен быть уникальным</strong>
			</p>
		{:else if state_nickname == StateNickname.IsTaken}
			<p class="mt-2 text-sm text-error">
				<strong>Ваш ник уже занятый</strong>
			</p>
		{:else if state_nickname == StateNickname.IsFree}
			<p class="mt-2 text-sm text-success">
				<strong>Ваш ник уникальный</strong>
			</p>
		{/if}
	</div>

	<button class="btn btn-secondary" disabled bind:this={registration_button} on:click={createAccout}
		>Создать новый акканут</button
	>
</form>
<form method="dialog" class="modal-backdrop">
	<button>close</button>
</form>
