<script lang="ts">
    import { Search, Button } from "flowbite-svelte";
    import type { SuggestDoc } from "$lib/apptypes/pdokresponses";
    import { mapstore, viewstore } from "$lib/stores";
	import type { Coordinate } from "ol/coordinate";
    import { zoekPdokLocatie, locatieCoordinate }  from "./pdoklocatieserverapi";

    let zoekterm: string;
    let suggestions: SuggestDoc[] = [];
    let zoomCoordinaten: Coordinate | undefined;

    $: if (zoomCoordinaten != undefined) {
        console.log(`set view center coordinaten ${zoomCoordinaten}`)
        $viewstore.setCenter(zoomCoordinaten)
        console.log($viewstore.getCenter())
        suggestions = []
        zoekterm = ""
        zoomCoordinaten = undefined
    }

    async function zoek(event: KeyboardEvent){
        suggestions = await zoekPdokLocatie(suggestions, zoekterm)
    }

</script>

<div class="relative flex flex-col items-start justify-center ">
    <form class="flex gap-2 mt-2" on:submit={async () => zoomCoordinaten = await locatieCoordinate(suggestions[0].id)}>
        <Search size="md" placeholder="Zoek op adres" 
            bind:value={zoekterm} 
            on:keyup={async (event) => zoek(event)}
        >
        </Search>
    </form>    
    {#if suggestions.length > 0}
    <div id="suggestions-popover" class="absolute transition duration-150 ease-in-out top-20 z-[999]">
        <div class="w-full bg-white rounded shadow-2xl">
            {#each suggestions as suggestion}
                <div class="flex p-2">
                    <button 
                        id={suggestion.id} 
                        on:click={async () => zoomCoordinaten = await locatieCoordinate(suggestion.id)}>
                        <p>{suggestion.weergavenaam}</p>
                    </button>
                </div>
                <hr>
            {/each}
        </div>
    </div>
    {/if}  
    
</div>

