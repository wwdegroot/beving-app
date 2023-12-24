<script lang="ts">
	import type { FeatureLike } from 'ol/Feature';

	export let feature: FeatureLike;
	export let index: number;
	export let max: number;

	const { date, year, depth, mag, place, time, ...rest } = feature.getProperties();
</script>

<div class="bg-white p-4 grid grid-cols-1">
	<div class=" row-span-1 flex min-w-80 justify-between">
		<div class="font-bold text-xl">{place} - {year}</div>
		<div class="font-italic">{index + 1} van {max}</div>
	</div>
	<div class="row-spawn-2 grid grid-rows-3">
		<div class="grid grid-cols-2">
			<span>Magnitude:</span><span>{mag}</span>
		</div>
		<div class="grid grid-cols-2">
			<span>Diepte:</span><span>{depth}</span>
		</div>
		<div class="grid grid-cols-2">
			<span>Datum:</span><span>{date}</span>
		</div>
		<div class="grid grid-cols-2">
			<span>Tijdstip:</span><span>{time}</span>
		</div>
	</div>
</div>
