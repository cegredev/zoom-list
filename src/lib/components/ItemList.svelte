<script lang="ts">
	import type { ComponentType } from 'svelte';

	export let component: ComponentType;
	/**
	 * IMPORTANT: These props have to fit the props for the given component!
	 */
	export let itemProps: any[];
</script>

<div class="wrapper">
	<div class="entries">
		{#each itemProps as item}
			<div class="row">
				<svelte:component this={component} {...item} />
			</div>
		{/each}
	</div>

	<div class="bottom-bar row bg-base-100">
		<slot />
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
		flex: 0;
		padding: 5px;
		padding-left: 15px;
	}
</style>
