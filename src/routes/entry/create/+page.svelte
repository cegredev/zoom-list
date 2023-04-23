<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { clientRecords, clientRecords_rust } from '$lib/stores';
	import { invoke } from '$lib/tauri';
	import ItemList from '$lib/components/ItemList.svelte';
	import CreatedEntryRow from './CreatedEntryRow.svelte';

	let allRecords = $clientRecords ?? [];
	let year = $page.url.searchParams.get('year');
	let month = $page.url.searchParams.get('month');

	async function submitData() {
		await invoke('submit_records', { records: $clientRecords_rust });

		await goto(`../entry?year=${year}&month=${month}`);
	}
</script>

<ItemList component={CreatedEntryRow} itemProps={allRecords}>
	<button class="btn btn-primary" on:click={submitData}>Hinzufügen</button>
</ItemList>
