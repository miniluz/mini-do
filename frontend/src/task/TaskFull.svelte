<script lang="ts">
	import type { Task } from "./task";

	export let task: Task;
	let task_copy = structuredClone(task);
	let date_string = new Date(
		task.due_time.getTime() - task.due_time.getTimezoneOffset() * 60000
	)
		.toISOString()
		.slice(0, -8);
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
						class="input input-sm xl:input-md input-bordered ml-4"
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
			>
			</div>
		</div>
	</div>
</div>
