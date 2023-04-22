<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';

	let path: string = 'hi';
	let divideByYear: boolean = false;
	let divideByMonth: boolean = false;

	$: {
		if (!divideByYear) {
			divideByMonth = false;
		}
	}
</script>

<div class="wrapper">
	<form method="post">
		<div>
			<label for="reportPath">Berichtsordner</label>
			<div class="input-group">
				<input bind:value={path} class="w-[100%]" type="text" name="reportPath" />
				<button
					type="button"
					class="btn"
					on:click={async () => {
						const newPath = await open({
							directory: true,
							multiple: false
						});

						if (!newPath) return;

						path = newPath.toString();
					}}>Öffnen</button
				>
			</div>
		</div>

		<div>
			<label class="label cursor-pointer">
				<span class="label-text">Unterordner für Jahr anlegen</span>
				<input bind:checked={divideByYear} type="checkbox" class="checkbox checkbox-lg" />
			</label>
		</div>

		<div>
			<label class="label cursor-pointer">
				<span class="label-text">Unterordner für Monat anlegen</span>
				<input
					bind:checked={divideByMonth}
					disabled={!divideByYear}
					type="checkbox"
					class="checkbox checkbox-lg"
				/>
			</label>
		</div>

		<div class="w-full flex flex-col items-center mt-auto mb-10">
			<button class="btn btn-success w-[50%]">Speichern</button>
		</div>
	</form>
</div>

<style>
	.wrapper {
		height: 100vh;
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	form {
		width: 80%;
		min-height: 100%;
		row-gap: 4%;
		display: flex;
		flex-direction: column;
	}

	form > div {
		@apply form-control;
	}

	input:not([type='checkbox']) {
		@apply input input-bordered;
	}

	label:not(.input-group) {
		@apply label;
	}

	button#save {
	}
</style>
