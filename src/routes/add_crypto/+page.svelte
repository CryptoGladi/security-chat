<script lang="ts">
	import { goto } from '$app/navigation';
	import LoadingCenter from '$lib/loading_center.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Icon from 'svelte-icons-pack';
	import IoAddCircle from "svelte-icons-pack/io/IoAddCircle";
	import IoRemoveCircle from "svelte-icons-pack/io/IoRemoveCircle";

	async function get_cryptos_for_accept() {
		return (await invoke('get_cryptos_for_accept')) as string[];
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

{#await get_cryptos_for_accept()}
	<LoadingCenter />
{:then nicknames}
	{#each nicknames as nickname}
		<div>
			<div class="flex">
				<div class="flex-1">
					<div class="px-1 py-1 flex w-full">
						<div class="placeholder avatar">
							<div class="bg-primary rounded-full w-24 avatar">
								<span class="text-6xl text">{nickname[0].toUpperCase()}</span>
							</div>
						</div>

						<div class="mx-4 flex flex-col w-full h-full">
							<div class=" items-start">
								<strong>{nickname}</strong>
							</div>
						</div>
					</div>
				</div>

				<div class="flex-none gap-2 flex-row">
					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<!-- svelte-ignore a11y-no-static-element-interactions -->
					<div on:click={() => {

					}}>
						<Icon src={IoAddCircle} color="green" size="50px"></Icon>
					</div>

					<!-- svelte-ignore a11y-click-events-have-key-events -->
					<!-- svelte-ignore a11y-no-static-element-interactions -->
					<div on:click={() => {
						
					}}>
						<Icon src={IoRemoveCircle} color="red" size="50px"></Icon>
					</div>
				</div>
			</div>
		</div>
	{/each}
{/await}
