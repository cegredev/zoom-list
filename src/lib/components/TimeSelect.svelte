<script lang="ts">
	import { fullMonths } from '$lib/uiconsts';

	export let year: number;
	export let month: number;
</script>

<input type="number" name="year" bind:value={year} class="input input-bordered w-32" />

<select name="month" class="select" bind:value={month}>
	{#each fullMonths as month, i}
		<option value={i + 1}>{month}</option>
	{/each}
</select>
