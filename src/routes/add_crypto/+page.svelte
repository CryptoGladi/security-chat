<script>
	import { goto } from "$app/navigation";

    async function get_cryptos_for_accept() {
        // TODO
    }
</script>

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
</div>