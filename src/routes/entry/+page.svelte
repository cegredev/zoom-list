<script lang="ts">
	import { DateTime, Duration } from 'luxon';
	import { fullMonths } from '$lib/uiconsts';
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api';
	import type { CSVData } from '$lib/types';
	import { goto } from '$app/navigation';
	import { csvDataStore } from '$lib/stores';

	const DATE_DISPLAY_FORMAT = 'dd.MM.yyyy';
	const DATE_STORE_FORMAT = 'dd.MM.yyyy HH:mm:ss';

	let date = DateTime.now();
	$: monthStart = date.startOf('month');
	$: totalDays = date.endOf('month').day;
	$: year = date.year;
	$: month = date.month;

	async function parseCSV(path: string) {
		const data: any = await invoke('parse_csv', { path });
		// let data: any;

		const result: CSVData = {
			records: data.records,
			floatingRecords: new Map()
		};

		for (const [key, value] of Object.entries(data.floating_records)) {
			console.log(key, value);
			result.floatingRecords.set(
				key,
				(value as any[]).map((value) => ({
					duration: Duration.fromDurationLike({ minutes: value.duration_minutes }),
					start: DateTime.fromFormat(value.start, DATE_STORE_FORMAT)
				}))
			);
		}

		// console.log([...result.floatingRecords.entries()]);
		csvDataStore.set(result);
		goto('/entry/create');
	}
</script>

<div class="wrapper">
	<div class="entries">
		{#each { length: totalDays } as _, days}
			<div class="row">
				<div class="badge badge-lg text-primary-content">
					{monthStart.plus({ days }).toFormat(DATE_DISPLAY_FORMAT)}
				</div>
				{#if Math.random() < 0.5}
					<div>Noch keine Daten vorhanden</div>
				{:else}
					<div>Einträge: {90 + Math.floor(Math.random() * 20)}</div>
					<button class="btn btn-sm btn-outline btn-error">Löschen</button>
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
				parseCSV(path);
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
