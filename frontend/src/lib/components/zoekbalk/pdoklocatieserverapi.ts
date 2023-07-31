import type { SuggestDoc, Result, LookupDoc, LookupResponse } from "$lib/apptypes/pdokresponses";
import type { Coordinate } from "ol/coordinate";


async function zoekPdokLocatie(suggestions: SuggestDoc[], zoekterm: string): Promise<SuggestDoc[]> {
    suggestions = []
    if (zoekterm.length >= 2) {
        const searchParams = new URLSearchParams({
            q: zoekterm,
            bq: "type:woonplaats^2",
            rows: "5"
        }).toString()
        const response = await fetch(
            "https://api.pdok.nl/bzk/locatieserver/search/v3_1/suggest?" + searchParams,
            {
                method: "GET",
                headers: { "Accept": "application/json" }
            }
        )
        const data: Result = await response.json()
        suggestions = data.response.docs as SuggestDoc[]

    }
    return suggestions

}

async function locatieCoordinate(suggestion_id: string): Promise<Coordinate | undefined> {
    let coordinate: Coordinate | undefined = undefined
    const searchParams = new URLSearchParams({
            id: suggestion_id
        }).toString()
    const response = await fetch(
        "https://api.pdok.nl/bzk/locatieserver/search/v3_1/lookup?" + searchParams,
        {
            method: "GET",
            headers: { "Accept": "application/json" }
        }
    )
    const data: Result = await response.json()
    if (data.response.numFound > 0){
        let lookup: LookupDoc = data.response.docs[0] as LookupDoc
        coordinate = getCoordinateFromWkt(lookup.centroide_rd)
    }

    return coordinate
}

const PATTERN = new RegExp(/\d{5,6}\.\d+\b|\d{5,6}\b/g)

function getCoordinateFromWkt(wktstring: string): Coordinate {
    
    let coordinateArray = wktstring.match(PATTERN)
    if (coordinateArray != null && coordinateArray.length == 2) {
        let coordinates: Coordinate = coordinateArray.map(n => Number(n))
        return coordinates
    } else {
        throw new Error(`Ongeldige WKT string, ${wktstring}`)
    }
    
}


export { zoekPdokLocatie, locatieCoordinate }