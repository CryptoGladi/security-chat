<script lang="ts">
    export let is_sender: boolean;
    export let text: string;
</script>

{#if is_sender}
<div class="chat chat-start">
<div class="chat-bubble">{text}</div>
</div>
{:else}
<div class="chat chat-end">
    <div class="chat-bubble">{text}</div>
</div>
{/if}