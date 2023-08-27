<script lang="ts">
	import { goto } from '$app/navigation';
	import LoadingCenter from '$lib/loading_center.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	onMount(async () => {
		console.log(1);
		goto('/welcom');
		console.log(2);
		let have_account = (await invoke('have_account')) as boolean;

		if (!have_account) {
			console.log(3);
			goto('/welcom');
		} else {
			console.log(4);
			goto('/main');
		}
	});
</script>

<LoadingCenter />
