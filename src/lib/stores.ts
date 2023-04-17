import { writable } from 'svelte/store';
import type { CSVData, ClientRecords } from './types';

export const clientRecords = writable<ClientRecords[] | null>(null);
export const clientRecords_rust = writable<any[] | null>(null);
