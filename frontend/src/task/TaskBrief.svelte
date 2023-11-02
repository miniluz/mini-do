<script lang="ts">
	import type { Task } from "./task";

	export let on_click: () => void;
	export let task: Task;
</script>

<button class="btn btn-neutral flex-col gap-1" on:click={on_click}>
	<p>{task.title}</p>
	<p class="text-xs text-neutral-content/80 font-semibold">
		{task.due_time.toLocaleDateString()}
	</p>
</button>
