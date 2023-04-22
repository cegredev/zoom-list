export function getInt(text: string | null, num: number): number {
	if (!text) return num;

	return parseInt(text) ?? num;
}
