<script lang="ts">
	export let data;

	import { goto } from '$app/navigation';
</script>

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

	<div class="flex-initial bg-neutral">
		<div class="flex flex-row">
			<textarea class="textarea" placeholder="BUI"></textarea>
			<button class="btn">ss</button>
		</div>
	</div>
</div>
