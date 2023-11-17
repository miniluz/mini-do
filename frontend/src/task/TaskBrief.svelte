<script lang="ts">
	export let on_click: () => void;
	export let title: string;
	export let due_time_string: string;
	export let is_selected: boolean = false;
	let extra_classes: string = "";
	console.log(title, is_selected);
	export { extra_classes as class };
</script>

<button
	class="btn btn-neutral flex-col gap-1 {is_selected
		? 'btn-primary'
		: ''} {extra_classes}"
	on:click={on_click}
>
	<p>{title}</p>
	<p class="text-xs text-neutral-content/80 font-semibold">
		{due_time_string}
	</p>
</button>
