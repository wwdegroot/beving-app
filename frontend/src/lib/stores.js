import { writable } from "svelte/store";
import Map  from 'ol/Map';


export const mapstore = writable({
    m: new Map()
});

