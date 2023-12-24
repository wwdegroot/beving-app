import { writable, type Writable } from "svelte/store";
import Map from 'ol/Map';
import View from 'ol/View';
import { getContext, setContext } from "svelte";

export const VIEW_CTX = Symbol('view');
export const MAP_CTX = Symbol('map');

export const viewstore = writable(new View())
export const mapstore = writable(new Map());



export function setMapStore(map: Map) {
    const mapStore = writable(map);
    setContext(MAP_CTX, mapStore);
    return mapStore
}

export function setViewStore(view: View) {
    const viewStore = writable(view);
    setContext(MAP_CTX, viewStore);
    return viewStore
}


export function getMapStore() {
    return getContext<Writable<Map>>(MAP_CTX)
}

export function getViewStore() {
    return getContext<Writable<View>>(VIEW_CTX)
}

