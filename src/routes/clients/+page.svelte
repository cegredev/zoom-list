<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import type { PageData } from './$types';
	import { fullMonths } from '$lib/uiconsts';
	import { DateTime } from 'luxon';
	import { getInt } from '$lib/util';
	import { page } from '$app/stores';

	export let data: PageData;
	const TODAY = DateTime.now();

	let year = getInt($page.url.searchParams.get('year'), TODAY.year);
	let month = getInt($page.url.searchParams.get('month'), TODAY.month);

	const unfocus = () => {
		// @ts-ignore
		document.activeElement.blur();
	};
</script>

<div class="wrapper">
	<div class="entries">
		{#each data.clients as client}
			<div class="row">
				<div class="badge badge-lg text-primary-content">
					{client.name}
				</div>
				<button class="btn btn-sm btn-outline btn-secondary">Bericht erstellen</button>
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
									await invoke('delete_client', { id: client.id });
									data.clients = data.clients.filter((c) => c.id !== client.id);
									unfocus();
								}}>Ja</button
							>
							<div class="divider divider-horizontal" />
							<button class="btn btn-sm btn-error" on:click={unfocus}>Nein</button>
						</div>
					</div>
				</div>
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
