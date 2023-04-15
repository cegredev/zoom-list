<script lang="ts">
	import { DateTime } from 'luxon';
	import type { PageData } from './$types';

	export let data: PageData;

	const DATE_DISPLAY_FORMAT = 'dd.MM.yyyy';

	let date = DateTime.now();
	$: monthStart = date.startOf('month');
	$: totalDays = date.endOf('month').day;
</script>

<div class="wrapper">
	{#each { length: totalDays } as _, days}
		<div>
			<div>{monthStart.plus({ days }).toFormat(DATE_DISPLAY_FORMAT)}</div>
			{#if Math.random() < 0.5}
				<div>Noch keine Daten vorhanden</div>
			{:else}
				<button>Löschen</button>
			{/if}
		</div>
	{/each}
	<div class="bottom-bar">
		<div>Jahr</div>
		<div>Monat</div>
		<button>Hinzufügen</button>
	</div>
</div>

<style>
	div.wrapper > div {
		display: flex;
		flex-direction: row;
	}

	.bottom-bar {
		background-color: white;
		position: fixed;
		bottom: 0;
		width: 100%;
		padding: 5px;
	}
</style>
