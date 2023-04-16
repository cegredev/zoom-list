import type { Client, ID } from './types';

export async function getClientMap(): Promise<Map<string, ID>> {
	return new Map();
}
