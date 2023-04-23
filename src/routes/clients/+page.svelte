<script lang="ts">
	import { invoke } from '$lib/tauri';
	import type { PageData } from './$types';
	import { fullMonths } from '$lib/uiconsts';
	import { DateTime } from 'luxon';
	import { getInt } from '$lib/util';
	import { page } from '$app/stores';
	import DeleteButton from '$lib/components/DeleteButton.svelte';

	export let data: PageData;
	const TODAY = DateTime.now();

	let year = getInt($page.url.searchParams.get('year'), TODAY.year);
	let month = getInt($page.url.searchParams.get('month'), TODAY.month);
</script>

<div class="wrapper">
	<div class="entries">
		{#each data.clients as client}
			<div class="row">
				<div class="badge badge-lg text-primary-content">
					{client.name}
				</div>
				<button class="btn btn-sm btn-outline btn-secondary">Bericht erstellen</button>

				<DeleteButton
					onDelete={async () => {
						await invoke('delete_client', { id: client.id });
						data.clients = data.clients.filter((c) => c.id !== client.id);
					}}
				/>
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
