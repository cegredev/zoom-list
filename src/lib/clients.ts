import { invoke } from '@tauri-apps/api';
import type { Client, ID } from './types';

export async function insertClient(name: string): Promise<ID> {
	return await invoke('insert_client', { name });
}

export async function getClients(): Promise<Client[]> {
	return await invoke('get_clients');
}

export async function getClientMap(): Promise<Map<string, ID>> {
	return new Map((await getClients()).map((client) => [client.name, client.id]));
}
