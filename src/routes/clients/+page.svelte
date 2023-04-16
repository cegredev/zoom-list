<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import type { PageData } from './$types';

	export let data: PageData;

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
				<!-- <button class="btn btn-sm btn-outline btn-error">Löschen</button> -->
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
</div>

<style>
	.dropdown-content {
	}

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
</style>
