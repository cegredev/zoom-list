import { invoke } from '@tauri-apps/api';
import type { PageLoad } from './$types';

export const load = (async () => {
	const config: any = await invoke('read_config');

	return {
		path: config.path,
		divideByYear: config.divide_by_year,
		divideByMonth: config.divide_by_month
	};
}) satisfies PageLoad;
