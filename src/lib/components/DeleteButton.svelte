<script lang="ts">
	export let onDelete: () => Promise<void>;

	function unfocus() {
		// @ts-ignore
		document.activeElement.blur();
	}
</script>

<div class="dropdown">
	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<!-- svelte-ignore a11y-label-has-associated-control -->
	<label tabindex="0" class="btn btn-sm btn-outline btn-error">Löschen</label>

	<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
	<div
		id="dropdown"
		tabindex="0"
		class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52"
	>
		<div class="label">
			<span class="label-text">Wirklich löschen?</span>
		</div>
		<div class="flex flex-row justify-center">
			<button
				class="btn btn-sm btn-success"
				on:click={async () => {
					await onDelete();
					unfocus();
				}}>Ja</button
			>
			<div class="divider divider-horizontal" />
			<button class="btn btn-sm btn-error" on:click={unfocus}>Nein</button>
		</div>
	</div>
</div>
