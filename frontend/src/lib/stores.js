import { writable } from "svelte/store";
import Map  from 'ol/Map';
import View from 'ol/View';

export const viewstore = writable(new View())


export const mapstore = writable({
    m: new Map()
});

