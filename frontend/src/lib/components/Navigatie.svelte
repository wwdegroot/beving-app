<script>
    import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from 'flowbite-svelte'
    import Aardbeving from '$lib/icons/Aardbeving.svelte';
</script>

<div class="calc-height">
<Navbar let:hidden let:toggle color='indigo'>
    <NavBrand href="/index">
      <Aardbeving color='choco'/>
      <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
        Aardbevingen in Nederland
      </span>
    </NavBrand>
    <NavHamburger on:click={toggle} />
    <NavUl {hidden}>
      <NavLi href="/data">
        <span class="text-xl text-gray dark:text-white">
          Data
        </span>
      </NavLi>
      <NavLi href="/services">        
        <span class="text-xl text-gray dark:text-white">
          Services
        </span>
      </NavLi>
    </NavUl>
  </Navbar>
</div>
  
  