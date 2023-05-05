import type { PageLoad } from './$types'


const fetchInduced = (async () => {
  const response = await fetch('https://cdn.knmi.nl/knmi/map/page/seismologie/all_induced.json')
  return await response.json()
})



export const load: PageLoad = async ({fetch}) => {
  const response = await fetch('https://cdn.knmi.nl/knmi/map/page/seismologie/all_induced.json')
  const aardbevingen = await response.json()
  console.log(aardbevingen)
  return { aardbevingen };
}