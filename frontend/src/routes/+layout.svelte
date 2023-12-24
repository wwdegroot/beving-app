<script>
	import '../app.postcss';
	import Map from 'ol/Map';
	import Navigatie from '$lib/components/Navigatie.svelte';
	import { MAP_CTX } from '$lib/stores';
	import View from 'ol/View';
	import { onMount, setContext } from 'svelte';
	import { rdnewprojection } from '$lib/openlayers';
	import { baselayers } from '$lib/components/layers/baselayers';
	import { defaults as defaultControls } from 'ol/control/defaults';
	import { writable } from 'svelte/store';

	setContext(
		MAP_CTX,
		writable(
			new Map({
				controls: defaultControls({ zoom: false }),
				layers: baselayers,
				view: new View({
					center: [172660, 485300],
					zoom: 2,
					projection: rdnewprojection
				})
			})
		)
	);

	onMount(async () => {});
</script>

<Navigatie />

<slot />
