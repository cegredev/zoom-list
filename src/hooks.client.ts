import { appConfig } from '$lib/stores';
import type { AppConfig } from '$lib/types';
import { invoke } from '@tauri-apps/api';

console.log('Loading config...');
const config: AppConfig = await invoke('read_config');
appConfig.set(config);
console.log('Loaded!');
