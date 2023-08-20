<script lang="ts">
	import { goto } from '$app/navigation';

    export let data;
    console.log(data);
</script>

<div class="navbar bg-neutral text-neutral-content">
    <div class="flex-1">
        <a class="btn btn-ghost normal-case text-xl" on:click={() => {
            goto("/main");
        }}>Закрыть</a>
    </div>

    <div class="flex-none gap-2">
        <div class="placeholder avatar">
            <div class="bg-primary rounded-full avatar w-12">
                <span class="text-3xl text">{data.nickname[0].toUpperCase()}</span>
            </div>
        </div>
    </div>
</div>

<div class="chat chat-start">
    <div class="chat-bubble">It's over Anakin, <br/>I have the high ground.</div>
  </div>
  <div class="chat chat-end">
    <div class="chat-bubble">You underestimate my power!</div>
  </div>
  