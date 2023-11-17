import type { Task } from "./task";
import { env } from "$env/dynamic/public";

const address: string = env.PUBLIC_BACKEND_ADDRESS || "";

export async function get_tasks(): Promise<Task[]> {
	const address = env.PUBLIC_BACKEND_ADDRESS;
	const request = new Request(address + "/backend/tasks");
	var r = await fetch(request)
		.then((response) => {
			if (response.status === 200) {
				return response.json();
			} else {
				throw new Error("Something went wrong on API server!");
			}
		})
		.catch((error) => {
			console.error(error);
		}) as Task[];
	return r.map((task) => {
		task.due_time = new Date(task.due_time);
		task.creation_time = new Date(task.creation_time);
		return task;
	})
}

export async function create_task(task: Task): Promise<number> {
	const address = env.PUBLIC_BACKEND_ADDRESS;
	const headers = new Headers();
	headers.append("Content-Type", "application/json")
	const request = new Request(address + "/backend/task", {
		method: "POST",
		headers,
		body: JSON.stringify(task)
	})
	return fetch(request)
		.then((response) => {
			if (response.status === 200) {
				return response.json();
			} else {
				throw new Error("Something went wrong on API server!");
			}
		})
		.catch((error) => {
			console.error(error);
		});
}

export async function update_task(task: Task) {
	const address = env.PUBLIC_BACKEND_ADDRESS;
	const headers = new Headers();
	headers.append("Content-Type", "application/json")
	const request = new Request(address + "/backend/task", {
		method: "PATCH",
		headers,
		body: JSON.stringify(task)
	})
	fetch(request)
		.then((response) => {
			if (response.status === 200) {
				return;
			} else {
				throw new Error("Something went wrong on API server!");
			}
		})
		.catch((error) => {
			console.debug(error);
		});
}