<script lang="ts">
    import {user$} from "$lib/store";
</script>

<div>
    {#if $loggedIn$}
        <slot name="logged"/>
    {:else if !$loggedIn$}
        <slot name="not-logged"/>
    {:else}
        <slot name="loading"/>
    {/if}
</div>
