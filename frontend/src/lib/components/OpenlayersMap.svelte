<script lang="ts">
	import { onMount } from 'svelte';
	import 'ol/ol.css';
	import { getMapStore } from '$lib/stores';

	const map = getMapStore();

	export let height: number = 400;
	let mapTargetDiv: HTMLElement;

	onMount(async () => {
		$map.setTarget(mapTargetDiv);
	});
</script>

<div bind:this={mapTargetDiv} class="map" style=" height: {height}px;">
	<slot />
</div>

<style>
	.map {
		width: 100%;
	}
</style>
