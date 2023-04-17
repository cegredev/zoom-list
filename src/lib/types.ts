import type { DateTime, Duration } from 'luxon';

export type ID = number;

export interface Client {
	id: ID;
	name: string;
}

export interface ZoomEntry {
	client?: ID;
	start: DateTime;
	duration: Duration;
}

export interface FloatingRecord {
	start: DateTime;
	duration: Duration;
}

export interface Record extends FloatingRecord {
	client: ID;
}

export interface CSVData {
	records: Record[];
	floatingRecords: Map<string, FloatingRecord[]>;
}

export interface ClientRecords {
	id?: ID;
	name: string;
	records: FloatingRecord[];
}
