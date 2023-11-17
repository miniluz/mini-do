<script lang="ts">
	import { onMount } from "svelte";
	import TaskBrief from "../task/TaskBrief.svelte";
	import TaskFull from "../task/TaskFull.svelte";
	import { create_task, get_tasks, update_task } from "../task/requests";
	import type { Task } from "../task/task";
	let task_full: TaskFull | undefined;

	const default_task: Task = {
		id: -1,
		title: "",
		text: "",
		creation_time: new Date(Date.now()),
		due_time: new Date(Date.now()),
		done: false,
	};

	let task_list_promise: Promise<Task[]> = Promise.resolve([]);
	onMount(() => (task_list_promise = get_tasks()));

	let selected_task: Task | null = null;

	var creating: boolean = false;
	var operation: Promise<any> = (async () => {})();
	function select_task(task: Task, new_task: boolean) {
		if (selected_task === null) {
			selected_task = task;
			return;
		}
		selected_task = task;
		var return_task = task_full?.change(task);
		var op: Promise<any> = Promise.resolve();
		if (return_task !== undefined) {
			if (creating) op = create_task(return_task);
			else {
				op = update_task(return_task);
			}
		}
		creating = new_task;
		operation = (async () => {
			await op;
			task_list_promise = get_tasks();
		})();
	}
</script>

<div class="flex w-full h-full flex-row">
	<div class="drawer xl:drawer-open">
		<input id="task-list" type="checkbox" class="drawer-toggle" />
		<div class="drawer-content flex flex-col">
			{#if selected_task != null}
				<TaskFull bind:this={task_full} task={selected_task} />
			{:else}
				<div
					class="flex flex-col items-center mt-16 w-full h-full prose max-w-none xl:fixed xl:top-0 xl:left-0"
				>
					<h2>No task selected.</h2>
					<p class="xl:hidden">To select a task,</p>
					<label
						for="task-list"
						class="btn btn-secondary drawer-button xl:hidden"
					>
						Open the task list
					</label>
				</div>
			{/if}
			<label
				for="task-list"
				class="btn drawer-button flex-col md:flex-row fixed top-0 left-0 mt-12 xl:hidden rounded-l-none"
			>
				<div>Task</div>
				<div>list</div>
			</label>
		</div>
		<div class="drawer-side">
			<label
				for="task-list"
				aria-label="close sidebar"
				class="drawer-overlay"
			/>
			<div
				class="menu p-4 w-80 min-h-full bg-base-200 text-base-content gap-2"
			>
				{#await operation}
					<p>Waiting for operation...</p>
				{:then _}
					{#await task_list_promise}
						<p>Waiting for task list...</p>
					{:then task_list}
						<TaskBrief
							title="Create task"
							due_time_string=""
							class="btn-success"
							on_click={async () =>
								await select_task(
									structuredClone(default_task),
									true
								)}
						/>
						{#each task_list.sort((a, b) => b.due_time.getTime() - a.due_time.getTime()) as task}
							<TaskBrief
								title={task.title}
								due_time_string={task.due_time.toLocaleDateString()}
								is_selected={task === selected_task}
								on_click={async () =>
									await select_task(task, false)}
							/>
						{/each}
					{:catch}
						<p>Error loading task list</p>
					{/await}
				{:catch}
					<p>Error performing operation.</p>
				{/await}
			</div>
		</div>
	</div>
</div>
