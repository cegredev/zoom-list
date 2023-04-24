// import { invoke as invokeTauri } from '@tauri-apps/api/tauri';

let _invoke: any;
let _open: any;

async function fixTauriFunctions() {
	const { invoke } = await import('@tauri-apps/api');
	_invoke = invoke;

	const { open } = await import('@tauri-apps/api/dialog');
	_open = open;
}

// There is a bug with Vite that forces it to use SSR even if disabled
// That in turn causes problems with Tauri APIs which rely on the window,
// which is why we have to do all of the weirdness in this file.
if (!import.meta.env.SSR) {
	await fixTauriFunctions();
}

export async function invoke(cmd: string, options?: any): Promise<any> {
	return await _invoke(cmd, options);
}

export async function open(options?: any): Promise<string[] | string | null> {
	return await _open(options);
}
