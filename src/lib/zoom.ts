import { readTextFile } from '@tauri-apps/api/fs';

export const readCSVFile = async (path: string) => {
	try {
		const content = await readTextFile(path);

		return content;
	} catch (error) {
		console.error(error);
	}
};
