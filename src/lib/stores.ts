import { writable } from 'svelte/store';
import type { ClientRecords } from './types';

export const clientRecords = writable<ClientRecords[] | null>(null);
export const clientRecords_rust = writable<any[] | null>(null);
