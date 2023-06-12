<script>
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger, Modal, Button } from 'flowbite-svelte'
    import Aardbeving from '$lib/icons/Aardbeving.svelte';
    let defaultModal = false;
</script>

<div class="calc-height m-h-12">
<Navbar let:hidden let:toggle color='indigo'>
    <NavBrand href="/">
      <Aardbeving color='choco'/>
      <span class="self-center whitespace-nowrap text-lg font-semibold dark:text-white sm:text-xl">
        Aardbevingen in Nederland
      </span>
    </NavBrand>
    <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
      <NavLi>
        <button on:click={() => defaultModal = true}>
          <span class="text-md md:text-xl text-gray dark:text-white">
            Data
          </span>
        </button>
      </NavLi>
    </NavUl>
  </Navbar>
</div>
  
<Modal title="Data" bind:open={defaultModal} autoclose outsideclose>
  <p class="text-base leading-relaxed text-gray-500 dark:text-gray-400">
    De bron van de getoonde data is de 
    <a class="underline" href="https://www.commissiemijnbouwschade.nl/begrippen/a/aardbeving-geinduceerd">geïnduceerde</a>
    bevings data uit 
    <a class="underline" href="https://www.knmi.nl/kennis-en-datacentrum/dataset/aardbevingscatalogus" target="_blank">aardbevingscatalogus</a>
    van het KNMI.
  </p>

  <svelte:fragment slot='footer'>
    <Button color="alternative">Sluit</Button>
  </svelte:fragment>
</Modal>
