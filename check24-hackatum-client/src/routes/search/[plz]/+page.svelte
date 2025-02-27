<script>
    import Button from '@smui/button';
    import CircularProgress from '@smui/circular-progress';
    import { Label } from '@smui/common';
    import { fetch_craftsmen } from '$lib/api_utils.js';
    import { admin_mode } from '$lib/stores.js';
    import { goto } from '$app/navigation';
    import { fade } from 'svelte/transition';

    export let data;
    let sort_by = 'default';
    let page_counter = 0;
    let loading = true;

    let craftsmen = [];

    function load_state_update(state) {
        loading = state;
    }

    async function load_craftsmen_wrapper(plz, page, sort_by) {
        if(typeof window !== 'undefined') {
            craftsmen = await fetch_craftsmen(window.location.origin.split(":").slice(0, 2).join(':'), plz, page, sort_by, load_state_update);
        }
    }

    $: load_craftsmen_wrapper(data.plz, page_counter, sort_by);
    $: sort_by, page_counter=0;
</script>

<svelte:head>
    <!-- Custom Background -->
    <style>
        body {
            background: transparent;
        }
        body:before {
            content: "";
            position: fixed;
            width: 100%;
            height: 100%;
            background-image: url('/img/bg.png?enhanced');
            background-size: cover;
            z-index: -1;
            filter: blur(10px);
            transform: scale(1.1);
        }
    </style>
</svelte:head>

<div class="flex flex-col md:flex-row items-start md:items-center justify-between">
    <h1 
        class="text-2xl text-white font-bold" 
        style="filter: drop-shadow(0px 0px 12px #063773)">Zu Ihrer Postleitzahl ({data.plz}) passende Profis
    </h1>

    <div class="mt-3 md:mt-0">
        <label for="sort" class="text-white mr-2 font-bold" style="filter: drop-shadow(0px 0px 12px #063773)"> Sortieren nach </label>
        <select class="text-white bg-24-blue rounded-md p-[0.35rem]" bind:value={sort_by}>
            <option value="default">Standard</option>
            <option value="distance">Entfernung</option>
            <option value="profile">Profilbewertung</option>
        </select>
    </div>
</div>

<!-- TODO: show number of *actual* results -->
{#if !loading}
    <span 
        class="text-base text-white mb-3 block drop-shadow-md" 
        style="filter: drop-shadow(0px 0px 10px #063773)">{craftsmen.length} Ergebnis{craftsmen.length !== 1 ? "se" : ""}
    </span>
{/if}

<div class="flex flex-col gap-2">
    {#each craftsmen as craftsman}
        <div class="relative flex flex-col gap-1 bg-white rounded-md p-2">
            <div class="flex flex-col sm:flex-row gap-1 xs:max-sm:justify-center sm:items-center">
                <div class="inline-flex flex-row gap-1 items-center">
                    <span class="py-1 px-2 bg-24-blue text-white rounded-md mr-1">{craftsman.profile_score.toFixed(1)}</span>
                    
                    <span>{craftsman.name}</span>
                </div>
        
                <div class="inline-grid w-min" style="--rating: {craftsman.profile_score / 0.03}%;">
                    <div class="text-gray-400 col-[1] row-[1] w-full text-lg">
                        <span>★★★★★</span>
                    </div>
                    <div class="text-24-yellow col-[1] row-[1] overflow-hidden w-[--rating] text-lg">
                        <span>★★★★★</span>
                    </div>
                </div>
            </div>
            <div class="flex flex-row items-center gap-1">
                <span class="material-symbols-rounded text-lg">location_on</span>
                <span class="text-sm">{Math.round(craftsman.distance * 10) / 10} km entfernt</span>
            </div>
            {#if $admin_mode}
                <button transition:fade={{duration: 100}} on:click={() => {goto('/update/' + craftsman.id)}} class="py-1 px-2 bg-24-blue rounded-bl-md rounded-tr-md absolute top-0 right-0">
                    <span class="material-symbols-rounded text-lg text-white">edit_square</span>
                </button>    
            {/if}
        </div>
    {:else}
        {#if loading}
        <div class="flex flex-row justify-center my-5">
            <CircularProgress style="height: 32px; width: 32px;" indeterminate />
        </div>    
        {:else}
            <span class="bg-white rounded-md p-2 w-max">Es wurden leider keine Profis für die PLZ "{data.plz}" gefunden.</span>
        {/if}
    {/each}
</div>
<!-- TODO: Only show if more pages available -->
{#if craftsmen.length > 0}
    {#if loading}
    <div class="flex flex-row justify-center mt-5">
        <CircularProgress style="height: 64px; width: 64px;" indeterminate />
    </div>
    {/if}
    <div class="flex justify-center">
        <Button touch variant="raised" class="bg-24-blue mt-3 p-5 rounded-full overflow-hidden" on:click={() => page_counter++}>
            <Label class="text-sm">Mehr anzeigen</Label>
        </Button>
    </div>
{/if}