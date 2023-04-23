<script lang="ts">
	import DeleteButton from '$lib/components/DeleteButton.svelte';
	import type { Client } from '$lib/types';

	export let client: Client;
	export let onDelete: () => Promise<void>;
</script>

<div class="badge badge-lg text-primary-content">
	{client.name}
</div>

<button class="btn btn-sm btn-outline btn-secondary">Bericht erstellen</button>

<DeleteButton {onDelete} />
