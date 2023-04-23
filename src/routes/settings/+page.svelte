<script lang="ts">
	import { invoke, open } from '$lib/tauri';
	import { appConfig } from '$lib/stores';

	$: {
		if (!$appConfig.divideByYear) {
			$appConfig.divideByMonth = false;
		}
	}
</script>

<div class="outer">
	<div class="inner">
		<div>
			<label for="reportPath">Berichtsordner</label>
			<div class="input-group">
				<input bind:value={$appConfig.path} class="w-[100%]" type="text" name="reportPath" />
				<button
					class="btn"
					on:click={async () => {
						const newPath = await open({
							directory: true,
							multiple: false
						});

						if (!newPath) return;

						$appConfig.path = newPath.toString();
					}}>Öffnen</button
				>
			</div>
		</div>

		<div>
			<label class="label cursor-pointer">
				<span class="label-text">Unterordner für Jahr anlegen</span>
				<input
					bind:checked={$appConfig.divideByYear}
					type="checkbox"
					class="checkbox checkbox-lg"
				/>
			</label>
		</div>

		<div>
			<label class="label cursor-pointer">
				<span class="label-text">Unterordner für Monat anlegen</span>
				<input
					bind:checked={$appConfig.divideByMonth}
					disabled={!$appConfig.divideByYear}
					type="checkbox"
					class="checkbox checkbox-lg"
				/>
			</label>
		</div>

		<div class="w-full flex flex-col items-center mt-auto mb-10">
			<button
				class="btn btn-success w-[50%]"
				on:click={async () => {
					await invoke('write_config', { config: $appConfig });
				}}>Speichern</button
			>
		</div>
	</div>
</div>

<style>
	.outer {
		height: 100vh;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.inner {
		width: 80%;
		min-height: 100%;
		row-gap: 4%;
		display: flex;
		flex-direction: column;
	}

	.inner > div {
		@apply form-control;
	}

	input:not([type='checkbox']) {
		@apply input input-bordered;
	}

	label:not(.input-group) {
		@apply label;
	}
</style>
