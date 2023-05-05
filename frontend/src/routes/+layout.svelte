<script>
	import '../app.postcss';
	import Navigatie from '$lib/components/Navigatie.svelte';
	import { browser } from '$app/environment'
	import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query'

	const queryClient = new QueryClient({
		defaultOptions: {
		queries: {
			enabled: browser,
		},
		},
	})
</script>
<Navigatie></Navigatie>
<QueryClientProvider client={queryClient}>
	<slot />
</QueryClientProvider>

