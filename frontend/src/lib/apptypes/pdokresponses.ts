export type Result = {
    response:     SuggestResponse | LookupResponse;
    highlighting: { [key: string]: Highlighting };
    spellcheck:   Spellcheck;
}

export type Highlighting = {
    suggest: string[];
}

export type SuggestResponse = {
    numFound:      number;
    start:         number;
    maxScore:      number;
    numFoundExact: boolean;
    docs:          SuggestDoc[];
}

export type SuggestDoc = {
    type:         string;
    weergavenaam: string;
    id:           string;
    score:        number;
}

export type LookupResponse = {
    numFound:      number;
    start:         number;
    maxScore:      number;
    numFoundExact: boolean;
    docs:          LookupDoc[];
}

export type LookupDoc = {
    bron:               string;
    identificatie:      string;
    provinciecode:      string;
    woonplaatscode:     string;
    type:               string;
    woonplaatsnaam:     string;
    provincienaam:      string;
    centroide_ll:       string;
    gemeentecode:       string;
    rdf_seealso:        string;
    weergavenaam:       string;
    provincieafkorting: string;
    centroide_rd:       string;
    id:                 string;
    gemeentenaam:       string;
}


export type Spellcheck = {
    suggestions: any[];
    collations:  any[];
}