<script lang="ts">
	import type { LayoutData } from './$types';
	import '$lib/global.css';

	export let data: LayoutData;
</script>

<div class="flex flex-row h-full">
	<nav class="flex flex-col bg-base-300 text-xl">
		<a href="/entry">Einträge</a>
		<a href="/clients">Personen</a>
		<a href="/settings">Einstellungen</a>
	</nav>
	<div class="w-full bg-base-200">
		<slot />
	</div>
</div>

<style>
	nav a {
		padding: 10px;
		border-bottom: 2px solid hsl(var(--b2));
	}

	nav a:last-of-type {
		border: none;
	}
</style>
