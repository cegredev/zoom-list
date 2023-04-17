<script lang="ts">
	import { DateTime, Duration } from 'luxon';
	import { fullMonths } from '$lib/uiconsts';
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api';
	import type { ClientRecords } from '$lib/types';
	import { goto } from '$app/navigation';
	import { clientRecords, clientRecords_rust } from '$lib/stores';
	import type { PageData } from './$types';

	export let data: PageData;

	const DATE_DISPLAY_FORMAT = 'dd.MM.yyyy';
	const DATE_STORE_FORMAT = 'dd.MM.yyyy HH:mm:ss';

	let date = DateTime.now();
	$: monthStart = date.startOf('month');
	$: year = date.year;
	$: month = date.month;

	const unfocus = () => {
		// @ts-ignore
		document.activeElement.blur();
	};

	async function parseCSV(path: string): Promise<ClientRecords[]> {
		const data: any[] = await invoke('parse_csv', { path });

		clientRecords_rust.set(data);

		return data.map((clientRecords) => ({
			id: clientRecords.id,
			name: clientRecords.name,
			records: clientRecords.records.map((record: any) => ({
				start: DateTime.fromFormat(record.start, DATE_STORE_FORMAT),
				duration: Duration.fromDurationLike({ minutes: record.duration_minutes })
			}))
		}));
	}
</script>

<div class="wrapper">
	<div class="entries">
		{#each data.recordCounts as count, days}
			<div class="row">
				<div class="badge badge-lg text-primary-content">
					{monthStart.plus({ days }).toFormat(DATE_DISPLAY_FORMAT)}
				</div>
				{#if count === 0}
					<div>Noch keine Daten vorhanden</div>
				{:else}
					<div>Einträge: {count}</div>

					<div class="dropdown">
						<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
						<!-- svelte-ignore a11y-label-has-associated-control -->
						<label tabindex="0" class="btn btn-sm btn-outline btn-error">Löschen</label>

						<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
						<div
							id="dropdown"
							tabindex="0"
							class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52"
						>
							<div class="label">
								<span class="label-text">Wirklich löschen?</span>
							</div>
							<div class="flex flex-row justify-center">
								<button
									class="btn btn-sm btn-success"
									on:click={async () => {
										await invoke('delete_records_on', { year, month, day: days });

										data.recordCounts = [...data.recordCounts];
										data.recordCounts[days] = 0;

										unfocus();
									}}>Ja</button
								>
								<div class="divider divider-horizontal" />
								<button class="btn btn-sm btn-error" on:click={unfocus}>Nein</button>
							</div>
						</div>
					</div>
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
				goto('/entry/create');
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
