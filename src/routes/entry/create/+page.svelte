<script lang="ts">
	import { goto } from '$app/navigation';
	import { clientRecords, clientRecords_rust } from '$lib/stores';
	import { ONLY_TIME_FORMAT_DETAILED } from '$lib/uiconsts';
	import { invoke } from '@tauri-apps/api';

	let allRecords = $clientRecords ?? [];

	async function submitData() {
		await invoke('submit_records', { records: $clientRecords_rust });

		goto('/entry');
	}
</script>

<div class="wrapper">
	<div class="entries">
		{#each allRecords as { name, id, records }}
			<div class="row">
				{#if !id}
					<div class="badge badge-success gap-2">Neu</div>
				{/if}
				<div class="badge badge-lg text-primary-content">{name}</div>
				<div>
					{#each records as record}
						<div>
							{record.start.toFormat(ONLY_TIME_FORMAT_DETAILED)} bis {record.start
								.plus(record.duration)
								.toFormat(ONLY_TIME_FORMAT_DETAILED)} Uhr | {record.duration.minutes} Minuten
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
	<div class="bottom-bar row">
		<!-- <span>Erkanntes Datum: {data.star}</span> -->
		<button class="btn btn-primary" on:click={submitData}>Hinzufügen</button>
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
		gap: 30px;
		align-items: flex-start;

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
