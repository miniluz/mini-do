export interface Task {
	id: number,
	title: string,
	text: string,
	creation_time: Date,
	due_time: Date,
	done: boolean,
}