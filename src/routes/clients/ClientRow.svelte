<script lang="ts">
	import DeleteButton from '$lib/components/DeleteButton.svelte';
	import { invoke } from '$lib/tauri';
	import type { Client } from '$lib/types';

	export let client: Client;
	export let onDelete: () => Promise<void>;
</script>

<div class="badge badge-lg text-primary-content">
	{client.name}
</div>

<button
	class="btn btn-sm btn-outline btn-secondary"
	on:click={async () => {
		const result = await invoke('generate_report', { clientId: client.id, year: 2023, month: 4 });

		console.log('result:', result);
	}}>Bericht erstellen</button
>

<DeleteButton {onDelete} />
