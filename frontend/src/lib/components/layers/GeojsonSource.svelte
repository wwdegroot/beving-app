<script lang="ts">
    import GeoJSON from 'ol/format/GeoJSON.js';
    import type Map from 'ol/Map.js';
    import WebGLPointsLayer from 'ol/layer/WebGLPoints';
    import VectorSource from 'ol/source/Vector.js';
    import {mapkey, rdnewprojection} from '$lib/openlayers.js'
    import { onMount, getContext, onDestroy } from 'svelte';
	import type { Point } from 'ol/geom';
	import Control from 'ol/control/Control';
    import noUiSlider from 'nouislider';
    import PipsFilter from 'nouislider';
    import 'nouislider/dist/nouislider.css';


    const { getMap } = getContext(mapkey);
    let map: Map = getMap();

    let jaarSliderElement: HTMLElement;
    let jaarSlider: HTMLElement;
    let jaarSliderControl: Control;


    const webglStyle =  {
    symbol: {
        symbolType: 'circle',
        size: ['interpolate', ['linear'], ['get', 'mag'], 0, 4, 5, 64],
        color: ['interpolate', ['linear'], ['get', 'mag'], 0, 'yellow', 3, 'darkred'],
        opacity: 0.75,
    },
    };

    const vectorSource = new VectorSource<Point>({
            url: 'http://localhost:8080/api/induced/geojson/query?start_year=2022&end_year=2024',
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
        noUiSlider.create(jaarSlider, {
        // Create two timestamps to define a range.
            range: {
                min: timestamp('1950'),
                max: timestamp('2050')
            },

        // Steps of one year
            step: 52 * 7 * 24 * 60 * 60 * 1000,

        // Two more timestamps indicate the handle starting positions.
            start: [timestamp('2000'), timestamp('2023')],
        // pips met jaartallen
            pips: {
                mode: 'positions',
                values: [1925, 1950, 1975, 2000, 2025],
                density: 10,
                filter: (value: number) => {
                    let decade = value / 10
                    if (decade % 1 === 0) {
                        return 1
                    } else {
                        return -1
                    }
                },
                format: {
                    from: (value: string) => timestamp(value), 
                    to: (value: number) => {
                        let d = new Date(value)
                        return d.getFullYear()
                    }
                }
            },
        // No decimals
            format: {
                from: (value: string) => Number(Number(value).toFixed(0)),
                to: (value: number) => value.toFixed(0)
            }
        })
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
    <div bind:this={jaarSlider}>

    </div>
</div>


<style>


    @media only screen and (max-width: 640px) {
        .jaar-slider-parent{
            background-color: rgba(255, 255, 255, 0.8);
            position: relative;
            padding-top: 0.5em;
            padding-left: 1.25em;
            padding-right: 1.25em;
            padding-bottom: 3.25em;
            border-radius: 5px;
            margin-top: 0.5em;
            margin-left: 1em;
            margin-right: 1em;

        }

        :global(.noUi-value-horizontal){
            transform: translate(-50%, 50%) rotate(45deg);
        }
    }

    @media only screen and (min-width: 640px) {
        .jaar-slider-parent{
            background-color: rgba(255, 255, 255, 0.8);
            position: relative;
            padding-top: 0.5em;
            padding-left: 1.25em;
            padding-right: 1.25em;
            padding-bottom: 3.25em;
            border-radius: 5px;
            margin-top: 0.5em;
            margin-left: 3em;
            margin-right: 3em;

        }

        :global(.noUi-value-horizontal){
            transform: translate(-50%, 50%);
        }
    }

</style>