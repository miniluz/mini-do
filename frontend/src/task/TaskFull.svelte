<script lang="ts">
	import type { Task } from "./task";

	export let task: Task;

	let task_copy: Task = structuredClone(task);
	let date_string: string = task_copy.due_time.toLocaleString();
	$: timestamp = Date.parse(date_string);

	export function change(task: Task): Task | void {
		if (isNaN(timestamp)) return;

		var return_task = structuredClone(task_copy);
		return_task.due_time = new Date(timestamp);

		task_copy = structuredClone(task);
		date_string = task_copy.due_time.toLocaleString();

		return return_task;
	}
</script>

<div class="flex justify-center w-full h-full xl:fixed xl:top-0 xl:left-0">
	<div class="max-w-prose h-full overflow-scroll">
		<div class="flex flex-col gap-5 p-5 items-center">
			<textarea
				placeholder="Title"
				aria-label="Task title"
				class="textarea textarea-bordered text-2xl font-bold w-full"
				required
				bind:value={task_copy.title}
			/>

			<div class="flex items-center gap-2">
				<label>
					Due in
					<input
						type="datetime-local"
						class="input input-sm xl:input-md input-bordered ml-4 {isNaN(
							timestamp
						)
							? 'input-error'
							: ''}"
						bind:value={date_string}
						required
					/>
				</label>
				<div class="divider divider-horizontal" />
				<button
					class="btn w-40 btn-sm xl:btn-md {task_copy.done
						? 'btn-success'
						: 'btn-error'}"
					on:click={() => (task_copy.done = !task_copy.done)}
				>
					{task_copy.done ? "Done" : "Not done"}
				</button>
			</div>
			<div
				contenteditable
				data-placeholder="Description"
				aria-label="Task description"
				class="textarea textarea-bordered w-full overflow-auto empty:before:content-[attr(data-placeholder)]"
				bind:innerHTML={task_copy.text}
			/>
		</div>
	</div>
</div>
