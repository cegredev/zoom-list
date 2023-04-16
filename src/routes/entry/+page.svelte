<script lang="ts">
	import { DateTime } from 'luxon';
	import { fullMonths } from '$lib/uiconsts';

	const DATE_DISPLAY_FORMAT = 'dd.MM.yyyy';

	let date = DateTime.now();
	$: monthStart = date.startOf('month');
	$: totalDays = date.endOf('month').day;
	$: year = date.year;
	$: month = date.month;
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
					<button class="btn btn-sm btn-outline btn-secondary">Löschen</button>
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

		<button class="btn btn-primary" on:click={async () => {}}>Hinzufügen</button>
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
		border-bottom: 1px solid hsl(var(--b2));
	}

	.bottom-bar {
		@apply bg-base-200;

		flex: 0;
		padding: 5px;
		padding-left: 15px;
	}
</style>
