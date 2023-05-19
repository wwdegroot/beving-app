<script lang="ts">
    import { onMount } from 'svelte';
    import { mapstore } from "$lib/stores";
    import { baselayers } from "$lib/components/layers/baselayers";
    import { rdnewprojection } from "$lib/openlayers";
	import OpenlayersMap from '$lib/components/OpenlayersMap.svelte';
    import Map from 'ol/Map';
    import View from 'ol/View';
    import { defaults as defaultControls} from 'ol/control/defaults';
	import GeojsonSource from '$lib/components/layers/GeojsonSource.svelte';

    let mapheight: number;
    let constant: number = 5;
    let heightCalcClass: string = "calc-height"
    let map = $mapstore.m;
    let mounted: boolean = false;

    let handleMapSize = () => {
        let elems = document.querySelectorAll(`.${heightCalcClass}`);
        let windowHeight = window.innerHeight;
        let totalHeight = 0;
        for (let i = 0; i < elems.length; i++) {
            let tag = (elems[i] as HTMLElement);
            let style = getComputedStyle(tag);
            totalHeight += (tag.offsetHeight + parseInt(style.marginTop) + parseInt(style.marginBottom) + constant);
        }
        mapheight = windowHeight - totalHeight;
    };

    onMount(async () => {

        let view = new View({
            center: [162660, 465300],
            zoom: 3,
            projection: rdnewprojection,
        
        });

        map = new Map({
            controls: defaultControls({zoom: false}),
            layers: baselayers,
            view: view,
        
        });
        mounted = true;
        
        handleMapSize()
    })
</script>

<svelte:window
 on:resize={handleMapSize}
/>
{#if mounted}

    <OpenlayersMap map={map} height={mapheight}>
        <GeojsonSource></GeojsonSource>
    </OpenlayersMap>
{/if}