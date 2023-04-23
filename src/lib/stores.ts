import { writable } from 'svelte/store';
import type { ClientRecords, AppConfig } from '$lib/types';

export const appConfig = writable<AppConfig>();
export const clientRecords = writable<ClientRecords[] | null>(null);
export const clientRecords_rust = writable<any[] | null>(null);
