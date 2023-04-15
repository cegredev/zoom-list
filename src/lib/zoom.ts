import { readTextFile } from '@tauri-apps/api/fs';
import type { ZoomEntry } from './types';
import { DateTime, Duration } from 'luxon';

const DATE_FORMAT = 'dd.MM.yyyy HH:mm:ss';

export async function readCSVFile(path: string): Promise<ZoomEntry[] | null> {
	try {
		const content = await readTextFile(path);

		const entries: ZoomEntry[] = [];
		let firstLine = true;
		for (let line of content.split('\n')) {
			if (firstLine) {
				firstLine = false;
				continue;
			}

			line = line.trim();

			const [name, _email, start, _end, durationMinutes] = line.split(',');
			if (!name) break;

			entries.push({
				duration: Duration.fromMillis(parseInt(durationMinutes) * 60 * 1000),
				start: DateTime.fromFormat(start, DATE_FORMAT)
			});
		}
		console.log(entries);

		return entries;
	} catch (error) {
		console.error(error);
		return null;
	}
}
