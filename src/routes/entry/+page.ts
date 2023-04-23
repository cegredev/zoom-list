import { invoke } from '$lib/tauri';
import type { PageLoad } from './$types';

export const load = (async () => {
	const recordCounts: number[] = await invoke('get_record_counts_month', { year: 2023, month: 4 });

	return { recordCounts };
}) satisfies PageLoad;
