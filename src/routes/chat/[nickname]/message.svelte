<script lang="ts">
	export let is_sender: boolean;
	export let text: string;
</script>

<div class="chat" class:chat-start={is_sender} class:chat-end={!is_sender}>
	<div class="chat-bubble">{text}</div>
</div>
