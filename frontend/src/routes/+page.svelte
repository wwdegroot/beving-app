<script lang="ts">
	import { onMount } from 'svelte';
	import { getMapStore, getViewStore } from '$lib/stores';
	import { baselayers } from '$lib/components/layers/baselayers';
	import { rdnewprojection } from '$lib/openlayers';
	import OpenlayersMap from '$lib/components/OpenlayersMap.svelte';
	import Map from 'ol/Map';
	import View from 'ol/View';
	import { defaults as defaultControls } from 'ol/control/defaults';
	import GeojsonSource from '$lib/components/layers/GeojsonSource.svelte';
	import Identify from '$lib/components/identify/Identify.svelte';

	const view = getViewStore();
	const map = getMapStore();

	let mapheight: number;
	let constant: number = 5;
	let heightCalcClass: string = 'calc-height';
	let mounted: boolean = false;

	let handleMapSize = () => {
		let elems = document.querySelectorAll(`.${heightCalcClass}`);
		let windowHeight = window.innerHeight;
		let totalHeight = 0;
		for (let i = 0; i < elems.length; i++) {
			let tag = elems[i] as HTMLElement;
			let style = getComputedStyle(tag);
			totalHeight +=
				tag.offsetHeight + parseInt(style.marginTop) + parseInt(style.marginBottom) + constant;
		}
		mapheight = windowHeight - totalHeight;
	};

	onMount(async () => {
		mounted = true;

		handleMapSize();
	});
</script>

<svelte:window on:resize={handleMapSize} />
{#if mounted}
	<OpenlayersMap height={mapheight}>
		<GeojsonSource />
		<Identify identifyLayerId="induced" />
	</OpenlayersMap>
{/if}
