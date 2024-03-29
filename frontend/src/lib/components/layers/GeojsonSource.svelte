<script lang="ts">
	import GeoJSON from 'ol/format/GeoJSON.js';
	import WebGLPointsLayer from 'ol/layer/WebGLPoints';
	import VectorSource from 'ol/source/Vector.js';
	import { rdnewprojection } from '$lib/openlayers.js';
	import { onMount, onDestroy } from 'svelte';
	import Control from 'ol/control/Control';
	import RangeSlider from 'svelte-range-slider-pips';
	import type Point from 'ol/geom/Point';
	import { getMapStore, getViewStore } from '$lib/stores';
	import type { Geometry } from 'ol/geom';
	import type Feature from 'ol/Feature';

	let view = getViewStore();
	let map = getMapStore();

	let jaarSliderElement: HTMLElement;
	let jaarSliderControl: Control;

	let magSliderElement: HTMLElement;
	let magSliderControl: Control;

	let minYear: number = 2020;
	let maxYear: number = 2025;
	$: year_values = [minYear, maxYear];

	let minMag: number = 1.0;
	let maxMag: number = 4.0;

	$: mag_values = [minMag, maxMag];

	$: magColor = `hsl(${Math.round(60 - ((maxMag + minMag) / 2) * 17)}, 90%, 60%)`;

	const webglStyle = {
		variables: {
			minYear: minYear,
			maxYear: maxYear,
			minMag: minMag,
			maxMag: maxMag
		},
		filter: [
			'all',
			['between', ['get', 'year'], ['var', 'minYear'], ['var', 'maxYear']],
			['between', ['get', 'mag'], ['var', 'minMag'], ['var', 'maxMag']]
		],
		'circle-radius': ['interpolate', ['exponential', 2], ['get', 'mag'], 0, 4, 5, 64],
		'circle-fill-color': [
			'interpolate',
			['exponential', 1],
			['get', 'mag'],
			0,
			'yellow',
			1,
			'orange',
			3,
			'darkred',
			4,
			'red'
		],
		'circle-rotate-with-view': false,
		'circle-displacement': [0, 0],
		'circle-opacity': 0.75
	};

	const vectorSource = new VectorSource<Feature>({
		url: 'http://localhost:8080/api/induced/geojson',
		format: new GeoJSON({
			dataProjection: rdnewprojection
		}),
		attributions: ' KNMI'
	});

	const webglLayer = new WebGLPointsLayer({
		source: vectorSource,
		style: webglStyle
	});
	webglLayer.set('layer_id', 'induced');

	function refreshMap() {
		webglStyle.variables.minYear = year_values[0];
		webglStyle.variables.maxYear = year_values[1];
		webglStyle.variables.minMag = mag_values[0];
		webglStyle.variables.maxMag = mag_values[1];
		minMag = mag_values[0];
		maxMag = mag_values[1];
	}

	function animate() {
		$map.render();
		window.requestAnimationFrame(animate);
	}

	onMount(() => {
		$map.addLayer(webglLayer);
		jaarSliderControl = new Control({
			element: jaarSliderElement
		});
		magSliderControl = new Control({
			element: magSliderElement
		});

		$map.addControl(jaarSliderControl);
		$map.addControl(magSliderControl);
		// animate the map for filtering
		animate();
	});

	onDestroy(() => {
		$map.removeLayer(webglLayer);
		$map.removeControl(jaarSliderControl);
		$map.removeControl(magSliderControl);
	});
</script>

<div class="jaar-slider-parent" bind:this={jaarSliderElement}>
	<RangeSlider
		range
		pushy
		float
		pips
		pipstep={25}
		all="label"
		min={1950}
		max={2050}
		step={1}
		bind:values={year_values}
		on:change={() => refreshMap()}
	/>
</div>

<div
	class="mag-slider-parent"
	style="--range-handle-focus: {magColor}; --range-range: {magColor}; --range-range-inactive: {magColor}"
	bind:this={magSliderElement}
>
	<RangeSlider
		id="color-pips"
		range
		pushy
		float
		pips
		pipstep={5}
		all="label"
		min={0}
		max={4}
		step={0.1}
		vertical
		bind:values={mag_values}
		on:change={() => refreshMap()}
	/>
</div>

<style>
	@media only screen and (max-width: 640px) {
		.jaar-slider-parent {
			position: relative;
			margin-top: 2.5em;
			margin-left: 1em;
			margin-right: 1em;
		}
	}

	@media only screen and (min-width: 640px) {
		.jaar-slider-parent {
			position: relative;
			margin-top: 2.5em;
			margin-left: 3em;
			margin-right: 3em;
		}
	}

	.mag-slider-parent {
		position: relative;
		max-width: 50px;
		margin-left: 1em;
		margin-top: 4em;
	}
</style>
