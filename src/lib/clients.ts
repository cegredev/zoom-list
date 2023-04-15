import type { Client, ID } from './types';

const clients: Client[] = [
	{
		id: 1,
		name: 'Jamila Bender'
	},
	{
		id: 2,
		name: 'Bozanu Zinala'
	}
];

export async function getClients(): Promise<Client[]> {
	return clients;
}

export async function getClientMap(): Promise<Map<string, ID>> {
	return new Map((await getClients()).map((client) => [client.name, client.id]));
}
