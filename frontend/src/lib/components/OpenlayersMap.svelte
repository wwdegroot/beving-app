<script lang="ts">
    import { onMount, setContext } from "svelte";
    import { mapkey } from "$lib/openlayers";
    import "ol/ol.css";
    import type Map from "ol/Map";
  
   
    export let map: Map;
    export let height: number  = 400;
    let mapTargetDiv: HTMLElement;

    setContext(mapkey, {
        getMap: () => map
    });

    onMount(async () => {
        map.setTarget(mapTargetDiv)
    })


</script>

<div bind:this={mapTargetDiv} class="map" style=" height: {height}px;">
    <slot></slot>
</div>

<style>
    .map {
        width: 100%;
    }
</style>