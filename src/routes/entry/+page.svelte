<script lang="ts">
	import { DateTime } from 'luxon';
	import { invoke, open } from '$lib/tauri';
	import { goto } from '$app/navigation';
	import { clientRecords } from '$lib/stores';
	import { page } from '$app/stores';
	import { getInt } from '$lib/util';
	import { parseCSV } from '$lib/csv';
	import ItemList from '$lib/components/ItemList.svelte';
	import EntryRow from './EntryRow.svelte';
	import TimeSelect from '$lib/components/TimeSelect.svelte';

	const TODAY = DateTime.now();

	let year = getInt($page.url.searchParams.get('year'), TODAY.year);
	let month = getInt($page.url.searchParams.get('month'), TODAY.month);
	let recordCounts: number[] = [];

	$: monthStart = DateTime.fromFormat(`${year}-${month.toString().padStart(2, '0')}`, 'yyyy-MM');
	$: {
		invoke('get_record_counts_month', { year, month }).then((counts: any) => {
			recordCounts = counts;
		});
	}
</script>

<ItemList
	component={EntryRow}
	itemProps={recordCounts.map((count, day) => ({
		count,
		date: monthStart.plus({ days: day }),
		onDelete: async () => {
			await invoke('delete_records_on', { year, month, day });

			recordCounts = [...recordCounts];
			recordCounts[day] = 0;
		}
	}))}
>
	<TimeSelect bind:year bind:month />

	<div class="divider divider-horizontal" />

	<button
		class="btn btn-primary"
		on:click={async () => {
			const path = await open({
				filters: [
					{
						name: 'CSV',
						extensions: ['csv']
					}
				]
			});

			if (!path) return;

			// @ts-ignore
			const data = await parseCSV(path);

			clientRecords.set(data);

			await goto(`/entry/create?year=${year}&month=${month}`);
		}}>Hinzufügen</button
	>
</ItemList>
