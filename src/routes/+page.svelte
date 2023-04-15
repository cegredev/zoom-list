<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { readCSVFile } from '$lib/zoom';

	let content = 'test';

	async function openCSVFile(): Promise<string | null> {
		const path = await open({
			multiple: false,
			filters: [
				{
					name: 'CSV',
					extensions: ['csv']
				}
			]
		});

		return path as string | null;
	}

	async function processCSVFile() {
		const path = await openCSVFile();
		if (!path) return;

		const entries = await readCSVFile(path);
	}
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<p>{content}</p>
<button on:click={processCSVFile}>Öffnen</button>
<a href="/entry">Entry</a>
