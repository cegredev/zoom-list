import type { Client } from '$lib/types';
import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api';

export const load = (async () => {
	const clients: Client[] = await invoke('get_clients');

	return { clients };
}) satisfies PageLoad;
