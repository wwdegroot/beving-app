<script lang="ts">
	import type Map from 'ol/Map.js';

	import { onMount, onDestroy } from 'svelte';
	import Control from 'ol/control/Control';
	import type { Extent } from 'ol/extent';
	import type { Pixel } from 'ol/pixel';
	import type MapBrowserEvent from 'ol/MapBrowserEvent';
	import type { FeatureLike } from 'ol/Feature';
	import type { Coordinate } from 'ol/coordinate';
	import type Source from 'ol/source/Source';
	import type Layer from 'ol/layer/Layer';
	import type LayerRenderer from 'ol/renderer/Layer';
	import type VectorSource from 'ol/source/Vector';
	import IdentifyResult from './identifyResult.svelte';
	import { getViewStore, getMapStore } from '$lib/stores';

	const view = getViewStore();
	const map = getMapStore();

	export let toggleButton: boolean = false;
	export let identifyLayerId: string | undefined = undefined;

	let identifyLayerSource: VectorSource | null | undefined;

	let identifyControl: Control;
	let identifyElement: HTMLElement;

	let resultsControl: Control;
	let resultsElement: HTMLElement;

	let identifyActive: boolean = !toggleButton;
	let features: FeatureLike[];
	$: features = [];

	$: if (features.length != 0) {
		console.log('features', features);
	} else {
		console.log('geen features gevonden');
	}

	$: if (identifyActive == true) {
		$map.on('click', mapClickEvent);
	} else {
		$map.un('click', mapClickEvent);
	}

	function mapClickEvent(event: MapBrowserEvent<UIEvent>) {
		features = [];
		// identifyMapFeature(map, event.pixel);
		if (identifyLayerSource) {
			let zoomLevel: number | undefined = $map.getView().getZoom() ?? 8;
			identifySourceFeature(identifyLayerSource, event.coordinate, zoomLevel);
		}
	}

	const identifySourceFeature = (source: VectorSource, point: Coordinate, zoom: number) => {
		let extentZoom = 1000 / zoom;
		console.log('zoom', extentZoom);
		const extent: Extent = [
			point[0] - extentZoom,
			point[1] - extentZoom,
			point[0] + extentZoom,
			point[1] + extentZoom
		];
		console.log('extent', extent);
		features = source.getFeaturesInExtent(extent);
	};

	const identifyMapFeature = (map: Map, pixel: Pixel) => {
		map.forEachFeatureAtPixel(
			pixel,
			(feature: FeatureLike) => {
				features.push(feature);
			},
			{ hitTolerance: 50 }
		);
	};

	onMount(() => {
		if (identifyLayerId) {
			let allLayers = $map.getAllLayers();
			let identifyLayer: Layer<Source, LayerRenderer<any>> | undefined = allLayers.find(
				(layer) => layer.get('layer_id') === identifyLayerId
			);
			identifyLayerSource = identifyLayer?.getSource() as VectorSource;
		}

		identifyControl = new Control({
			element: identifyElement
		});
		if (toggleButton) {
			$map.addControl(identifyControl);
		}
		resultsControl = new Control({
			element: resultsElement
		});
		$map.addControl(resultsControl);
	});

	onDestroy(() => {
		if (toggleButton) {
			$map.removeControl(identifyControl);
		}
		$map.removeControl(resultsControl);
	});
</script>

{#if toggleButton}
	<button
		class="bg-white rounded-md shadow-lg text-2xl inset-1 top-10 left-10 p-1"
		bind:this={identifyElement}
		on:click={() => (identifyActive = !identifyActive)}
	>
		&#9432;
	</button>
{/if}

<div
	class="absolute bottom-4 left-4 bg-white max-w-lg max-h-60 rounded-md shadow-xl overflow-y-auto"
	bind:this={resultsElement}
>
	{#if features.length > 0}
		{#each features as feature, i}
			<IdentifyResult {feature} index={i} max={features.length} />
		{/each}
	{/if}
</div>
