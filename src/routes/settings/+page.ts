import { invoke } from '@tauri-apps/api';
import type { PageLoad } from './$types';

export const load = (async () => {
	const config: any = await invoke('read_config');

	return config;
}) satisfies PageLoad;
