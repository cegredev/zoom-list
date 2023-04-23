let invokeFunction: any = () => console.log('invoking nothing!');
let openFunction: any = () => console.log('opening nothing!');
let readTextFileFunction: any = () => console.log('reading nothing!');

async function fixTauriFunctions() {
	const { invoke } = await import('@tauri-apps/api');
	invokeFunction = invoke;

	const { open } = await import('@tauri-apps/api/dialog');
	openFunction = open;

	const { readTextFile } = await import('@tauri-apps/api/fs');
	readTextFileFunction = readTextFile;
}

if (!import.meta.env.SSR) {
	await fixTauriFunctions();
}

export async function invoke<T>(name: string, args?: any): Promise<T> {
	return await invokeFunction(name, args);
}

export async function open(options?: any): Promise<any> {
	return await openFunction(options);
}

export async function readTextFile(path: string, options?: any) {
	return await readTextFileFunction(path, options);
}
