<script lang="ts">
	import type { FloatingRecord, ID } from '$lib/types';
	import { ONLY_TIME_FORMAT_DETAILED } from '$lib/uiconsts';

	export let id: ID | undefined;
	export let name: string;
	export let records: FloatingRecord[];
</script>

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
