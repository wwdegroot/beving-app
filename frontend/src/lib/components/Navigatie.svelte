<script>
    import { Modal, Button } from 'flowbite-svelte'
    import Zoekbalk from './zoekbalk/Zoekbalk.svelte';
    import Aardbeving from '$lib/icons/Aardbeving.svelte';
    let defaultModal = false;
</script>

<div class="calc-height m-h-12">

  <nav class="border-gray-200 bg-gradient-to-r from-sky-100 to-blue-300 dark:bg-gray-800 dark:border-gray-700">
    <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
      <a href="/" class="flex items-center">
        <Aardbeving color="darkblue"/>
        <span class="self-center whitespace-nowrap text-lg font-semibold text-blue-900 dark:text-white sm:text-xl">
          Aardbevingen in Nederland
        </span>
      </a>
      <Zoekbalk/>
      <button class="bg-blue-100 p-2 rounded-md" on:click={() => defaultModal = true}>
        <span class="text-md md:text-xl md:font-bold text-blue-900 dark:text-white">
          Info
        </span>
      </button>
    </div>
  </nav>
</div>

<Modal title="Data" bind:open={defaultModal} autoclose outsideclose={true}>
  <p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
    De bron van de getoonde data is de 
    <a class="underline" href="https://www.commissiemijnbouwschade.nl/begrippen/a/aardbeving-geinduceerd" target="_blank">geïnduceerde</a>
    bevings data uit de
    <a class="underline" href="https://www.knmi.nl/kennis-en-datacentrum/dataset/aardbevingscatalogus" target="_blank">aardbevingscatalogus</a>
    van het KNMI.
  </p>

  <svelte:fragment slot='footer'>
    <Button color="alternative">Sluit</Button>
  </svelte:fragment>
</Modal>
