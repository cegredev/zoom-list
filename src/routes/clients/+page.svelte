<script lang="ts">
	import { invoke } from '$lib/tauri';
	import type { PageData } from './$types';
	import { DateTime } from 'luxon';
	import { getInt } from '$lib/util';
	import { page } from '$app/stores';
	import ItemList from '$lib/components/ItemList.svelte';
	import ClientRow from './ClientRow.svelte';
	import TimeSelect from '$lib/components/TimeSelect.svelte';

	export let data: PageData;
	const TODAY = DateTime.now();

	let year = getInt($page.url.searchParams.get('year'), TODAY.year);
	let month = getInt($page.url.searchParams.get('month'), TODAY.month);
</script>

<ItemList
	component={ClientRow}
	itemProps={data.clients.map((client) => ({
		client,
		onDelete: async () => {
			await invoke('delete_client', { id: client.id });
			data.clients = data.clients.filter((c) => c.id !== client.id);
		}
	}))}
>
	<TimeSelect bind:year bind:month />
</ItemList>
