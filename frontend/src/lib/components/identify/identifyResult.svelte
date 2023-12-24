<script lang="ts">
	import type { FeatureLike } from 'ol/Feature';

	export let feature: FeatureLike;
	const { date, year, depth, mag, place, time, ...rest } = feature.getProperties();
</script>

<div class="bg-white grid grid-rows-3">
	<div><p class="font-bold text-xl">{place} - {date} {time}</p></div>
	<div><span>Magnitude</span><span>{mag}</span></div>
	<div><span>Diepte</span><span>{depth}</span></div>
</div>
