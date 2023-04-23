import { DateTime, Duration } from 'luxon';
import { clientRecords_rust } from './stores';
import { invoke } from './tauri';
import type { ClientRecords } from './types';

export const DATE_STORE_FORMAT = 'dd.MM.yyyy HH:mm:ss';

export async function parseCSV(path: string): Promise<ClientRecords[]> {
	const data: any[] = await invoke('parse_csv', { path });
	clientRecords_rust.set(data);
	return data.map((clientRecords) => ({
		id: clientRecords.id,
		name: clientRecords.name,
		records: clientRecords.records.map((record: any) => ({
			start: DateTime.fromFormat(record.start, DATE_STORE_FORMAT),
			duration: Duration.fromDurationLike({ minutes: record.durationMinutes })
		}))
	}));
}
