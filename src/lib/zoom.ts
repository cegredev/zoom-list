import { readTextFile } from '$lib/tauri';
import type { ID, ZoomEntry } from './types';
import { DateTime, Duration } from 'luxon';
import { getClientMap } from './clients';
import { DATE_FORMAT_DETAILED } from './uiconsts';

export async function readCSVFile(path: string): Promise<ZoomEntry[] | null> {
	let content: string;
	try {
		content = await readTextFile(path);
	} catch (error) {
		console.error(error);
		return null;
	}

	const clientMap = await getClientMap();

	const entries: ZoomEntry[] = [];
	let firstLine = true;
	for (let line of content.split('\n')) {
		if (firstLine) {
			firstLine = false;
			continue;
		}

		line = line.trim();

		const [name, _email, start, _end, durationMinutesStr] = line.split(',');
		if (!name) break;

		entries.push({
			client: clientMap.get(name.trim()),
			duration: Duration.fromDurationLike({ minutes: parseInt(durationMinutesStr) }),
			start: DateTime.fromFormat(start, DATE_FORMAT_DETAILED)
		});
	}
	console.log(entries);

	return entries;
}

// export async function
