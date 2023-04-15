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
