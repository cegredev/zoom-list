<script lang="ts">
	import { csvDataStore } from '$lib/stores';
	import { DATE_FORMAT_DETAILED } from '$lib/uiconsts';
	import type { PageData } from './$types';

	export let data: PageData;

	$: floatingRecords = [...$csvDataStore!.floatingRecords.entries()];
</script>

{#each floatingRecords as [name, entries]}
	<div>
		<div>{name}</div>
		<div>
			{#each entries as entry}
				<div>{entry.start.toFormat(DATE_FORMAT_DETAILED)}</div>
				<div>{entry.duration.minutes}</div>
			{/each}
		</div>
	</div>
{/each}
