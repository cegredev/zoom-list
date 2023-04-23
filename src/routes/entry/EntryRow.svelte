<script lang="ts">
	import DeleteButton from '$lib/components/DeleteButton.svelte';
	import { ONLY_DATE_FORMAT } from '$lib/uiconsts';
	import type { DateTime } from 'luxon';

	export let date: DateTime;
	export let count: number;
	export let onDelete: () => Promise<void>;
</script>

<div class="badge badge-lg text-primary-content">
	{date.toFormat(ONLY_DATE_FORMAT)}
</div>
{#if count === 0}
	<div>Noch keine Daten vorhanden</div>
{:else}
	<div>Einträge: {count}</div>

	<DeleteButton {onDelete} />
{/if}
