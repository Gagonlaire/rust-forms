<script>
    import AuthGuard from "$lib/components/AuthGuard.svelte";
</script>

<div class="flex justify-between items-center w-full h-10 bg-red-300">
    <span>example header</span>
    <AuthGuard>
        <span slot="logged">logged</span>
        <span slot="not-logged">loading</span>
        <span slot="loading">fall back</span>
    </AuthGuard>
</div>
