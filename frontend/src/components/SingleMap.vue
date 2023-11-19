<template>
  <l-map :use-global-leaflet="false" v-model="zoomLvl" v-model:zoom="zoomLvl" :center="coords as any">
    <l-tile-layer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"></l-tile-layer>

    <l-circle-marker v-if="radius" :lat-lng="coords as any" :radius="radius" color="red" />
    <l-marker :lat-lng="coords as any" :z-index-offset="5000">
      <l-icon icon-url="marker-icon-2x.png" :icon-size="[25, 41]" />
    </l-marker>
  </l-map>
</template>

<script lang="ts">
import { LMap, LIcon, LTileLayer, LMarker, LCircleMarker } from "@vue-leaflet/vue-leaflet";
import "leaflet/dist/leaflet.css";

export default {
  components: {
    LMap,
    LTileLayer,
    LMarker,
    LCircleMarker,
    LIcon,
  },
  props: {
    coords: {
      type: Array<number>,
      required: true,
    },
    zoom: {
      type: Number,
      default: 10,
    },
    radius: {
      type: Number,
    },
  },
  data() {
    return {
      zoomLvl: this.$props.zoom,
    };
  },
};
</script>
