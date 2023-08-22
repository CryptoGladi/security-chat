<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	export let data;
	let div_messages: HTMLDivElement;
	let div_sender: HTMLDivElement;

	const height_chat = 87;
	const height_message = 100 - height_chat;

	onMount(async () => {
		div_messages.setAttribute("height", `${height_chat}vh`);
		//div_sender.setAttribute("height", `${height_message}vh`);
	});
</script>

<div class="navbar bg-neutral text-neutral-content">
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

<div class="flex flex-col">
	<div class="overflow-y-scroll h-[89vh]" bind:this={div_messages}>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
		<div class="chat chat-start">
			<div class="chat-bubble">It's over Anakin, <br />I have the high ground.</div>
		</div>
		<div class="chat chat-end">
			<div class="chat-bubble">You underestimate my power!</div>
		</div>
	</div>

	<div class="h-12 bg-slate-600" bind:this={div_sender}>

	</div>
</div>


