<script lang="ts">
	import DeleteButton from '$lib/components/DeleteButton.svelte';
	import { invoke } from '$lib/tauri';
	import type { Client } from '$lib/types';

	export let year: number, month: number;
	export let client: Client;
	export let onDelete: () => Promise<void>;
</script>

<div class="badge badge-lg text-primary-content">
	{client.name}
</div>

<button
	class="btn btn-sm btn-outline btn-secondary"
	on:click={async () => {
		const file = await invoke('generate_report', { clientId: client.id, year, month });

		console.log('File:', file);
	}}>Bericht erstellen</button
>

<DeleteButton {onDelete} />
