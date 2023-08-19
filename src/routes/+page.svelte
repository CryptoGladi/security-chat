<script lang="ts">
	import { goto } from '$app/navigation';
	import LoadingCenter from '$lib/loading_center.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	onMount(async () => {
		let have_account = (await invoke('have_account')) as boolean;

		if (!have_account) {
			goto('/welcom');
		} else {
			goto('/main');
		}
	});
</script>

<LoadingCenter></LoadingCenter>