<script lang="ts">
	import { goto } from '$app/navigation';
	import IoSend from 'svelte-icons-pack/io/IoSend';
	import Icon from 'svelte-icons-pack/Icon.svelte';
	import Message from './message.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import _ from 'lodash';
	import LoadingCenter from '$lib/loading_center.svelte';

	export let data;
	let text_message: string;

	class MessageInfo {
		constructor(text: string, is_sender: boolean) {
			this.text = text;
			this.is_sender = is_sender;
		}

		text: string;
		is_sender: boolean;
	}

	async function get_messages() {
		let my_nickname = (await invoke('get_nickname')) as string;
		let messages = (await invoke('get_messages_for_user', { nicknameFrom: data.nickname }) ) as any[];

		return _.reverse(_.map(messages, (n) => {
			return new MessageInfo(n.body.text, n.sender != my_nickname);
		}));
	}

	async function send_message() {
		invoke("send_message", { nickname: data.nickname, message: text_message });
	}
</script>

<div class="flex flex-col h-full">
	<div class="navbar bg-neutral text-neutral-content flex-initial">
		<div class="flex-1">
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<a
				class="btn btn-ghost normal-case text-xl"
				on:click={() => {
					goto('/main');
				}}>Закрыть</a
			>
		</div>

		<div class="flex-none gap-2">
			<div class="placeholder avatar">
				<div class="bg-primary rounded-full avatar w-12">
					<span class="text-3xl text">{data.nickname[0].toUpperCase()}</span>
				</div>
			</div>
		</div>
	</div>

	<div class="overflow-y-scroll flex-auto">
		{#await get_messages()}
			<LoadingCenter />
		{:then messages}
			{#each messages as message}
				<Message text={message.text} is_sender={message.is_sender} />
			{/each}
		{/await}
	</div>

	<div class="flex-initial bg-neutral">
		<div class="flex flex-row">
			<textarea class="textarea textarea-bordered !outline-none flex-auto" bind:value={text_message} placeholder="BUI"
			on:keydown={(e) => {
				if (e.key === "Enter") {
					// @ts-ignore
					send_message();
				}
			}}/>
			<button class="btn btn-info flex-initial btn-circle self-center" on:click={send_message}>
				<Icon src={IoSend} size="20" className="custom-icon" />
			</button>
		</div>
	</div>
</div>

<style lang="less">
	// FOR FLEXBOX!

	:global(html) {
		height: 100%;
		margin: 0;
	}

	:global(body) {
		height: 100%;
		margin: 0;
	}

	// FOR FLEXBOX!
</style>
