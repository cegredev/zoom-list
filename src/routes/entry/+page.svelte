<script lang="ts">
	import { DateTime } from 'luxon';
	import { fullMonths } from '$lib/uiconsts';
	import { invoke, open } from '$lib/tauri';
	import { goto } from '$app/navigation';
	import { clientRecords } from '$lib/stores';
	import { page } from '$app/stores';
	import { getInt } from '$lib/util';
	import { parseCSV } from '$lib/csv';
	import DeleteButton from '$lib/components/DeleteButton.svelte';

	const DATE_DISPLAY_FORMAT = 'dd.MM.yyyy';
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

<div class="wrapper">
	<div class="entries">
		{#each recordCounts as count, day}
			<div class="row">
				<div class="badge badge-lg text-primary-content">
					{monthStart.plus({ days: day }).toFormat(DATE_DISPLAY_FORMAT)}
				</div>
				{#if count === 0}
					<div>Noch keine Daten vorhanden</div>
				{:else}
					<div>Einträge: {count}</div>

					<DeleteButton
						onDelete={async () => {
							await invoke('delete_records_on', { year, month, day });

							recordCounts = [...recordCounts];
							recordCounts[day] = 0;
						}}
					/>
				{/if}
			</div>
		{/each}
	</div>
	<div class="bottom-bar row">
		<input type="number" name="year" bind:value={year} class="input input-bordered w-32" />

		<select name="month" class="select" bind:value={month}>
			{#each fullMonths as month, i}
				<option value={i + 1}>{month}</option>
			{/each}
		</select>

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
	</div>
</div>

<style>
	.wrapper {
		height: 100vh;
		display: flex;
		flex-direction: column;
	}

	.entries {
		flex: auto;
		overflow-y: auto;
	}

	.row {
		display: flex;
		flex-direction: row;
		gap: 10px;
		align-items: center;

		padding: 15px;
		border-bottom: 1px solid hsl(var(--b1));
	}

	.bottom-bar {
		@apply bg-base-100;

		flex: 0;
		padding: 5px;
		padding-left: 15px;
	}
</style>
