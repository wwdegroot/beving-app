<script lang="ts">
    import GeoJSON from 'ol/format/GeoJSON.js';
    import type Map from 'ol/Map.js';
    import WebGLPointsLayer from 'ol/layer/WebGLPoints';
    import VectorSource from 'ol/source/Vector.js';
    import {mapkey, rdnewprojection} from '$lib/openlayers.js'
    import { onMount, getContext, onDestroy } from 'svelte';
	import type { Point } from 'ol/geom';
	import Control from 'ol/control/Control';
    import RangeSlider from "svelte-range-slider-pips";

    const { getMap } = getContext(mapkey);
    let map: Map = getMap();

    let jaarSliderElement: HTMLElement;
    let jaarSliderControl: Control;

    let values: number[] = [2000, 2025]


    const webglStyle =  {
    symbol: {
        symbolType: 'circle',
        size: ['interpolate', ['linear'], ['get', 'mag'], 0, 4, 5, 64],
        color: ['interpolate', ['linear'], ['get', 'mag'], 0, 'yellow', 3, 'darkred'],
        opacity: 0.75,
    },
    };

    const vectorSource = new VectorSource<Point>({
            url: `http://localhost:8080/api/induced/geojson/query?start_year=${values[0]}&end_year=${values[1]}`,
            format: new GeoJSON({
                dataProjection: rdnewprojection,
            }),
        });
    
    const webglLayer = new WebGLPointsLayer({
        source: vectorSource,
        style: webglStyle
    });

    function timestamp(datestring: string) {
        return new Date(datestring).getTime();
    }

    

    onMount(() => {
        map.addLayer(webglLayer)

        jaarSliderControl = new Control({
            element: jaarSliderElement
        })
        map.addControl(jaarSliderControl);
    })

    onDestroy(() => {
        map.removeLayer(webglLayer)
        map.removeControl(jaarSliderControl)
    })
</script>

<div class="jaar-slider-parent" bind:this={jaarSliderElement}>
    <RangeSlider
        range
        pushy 
        float
        pips
        pipstep={25}
        all='label'
        min={1950} 
        max={2050} 
        step={1}
        bind:values
        on:change={() => {console.log(values)}}
    />
</div>


<style>

    @media only screen and (max-width: 640px) {
        .jaar-slider-parent{
            position: relative;
            margin-top: 2.5em;
            margin-left: 1em;
            margin-right: 1em;

        }

    }

    @media only screen and (min-width: 640px) {
        .jaar-slider-parent{
            position: relative;
            margin-top: 2.5em;
            margin-left: 3em;
            margin-right: 3em;

        }

    }

</style>