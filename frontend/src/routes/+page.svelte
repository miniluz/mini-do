<script lang="ts">
	import TaskBrief from "../task/TaskBrief.svelte";
	import TaskFull from "../task/TaskFull.svelte";
	import type { Task } from "../task/task";
	let task1: Task = {
		id: 0,
		title: "Go shopping",
		text: "You need to buy something for tomorrow!",
		creation_time: new Date(2023, 10, 25, 10, 58, 13),
		due_time: new Date(2023, 12, 26, 11, 58, 13),
		done: false,
	};
	let task2: Task = {
		id: 0,
		title: "Go study",
		text: "You need to study some subjects!",
		creation_time: new Date(2022, 11, 25, 10, 58, 13),
		due_time: new Date(2024, 12, 26, 10, 57, 13),
		done: true,
	};

	let task_list: Task[] = [task1, task2].sort(
		(a, b) => a.due_time.getTime() - b.due_time.getTime()
	);
	let selected_task: Task | null = null;

	$: console.log(selected_task?.title);
</script>

<div class="flex w-full h-full flex-row">
	<div class="drawer xl:drawer-open">
		<input id="task-list" type="checkbox" class="drawer-toggle" />
		<div class="drawer-content flex flex-col">
			{#if selected_task != null}
				<TaskFull bind:task={selected_task} />
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
				{#each task_list as task}
					<TaskBrief
						{task}
						on_click={() => {
							selected_task = task;
						}}
					/>
				{/each}
			</div>
		</div>
	</div>
</div>
