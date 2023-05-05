# Aardbevingen in Nederland Viewer

Web viewer die de aardbevingen in Nederland all geinduceerde bevingen toont.

## Backend

Axum Backend api die de [knmi aardbevingen catalogus](https://www.knmi.nl/kennis-en-datacentrum/dataset/aardbevingscatalogus) bevraagd tijdens opstarten, de data omzet naar geojson met rd_new coordinaten en deze ontsluit.

## Frontend

SvelteKit applicatie die met openlayers de data op de kaart toont.