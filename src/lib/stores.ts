import { writable } from 'svelte/store';
import type { CSVData } from './types';

export const csvDataStore = writable<CSVData | null>(null);
