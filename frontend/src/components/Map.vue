<template>
  <div style="height: 75vh; width: 50vw;">
    <l-map :use-global-leaflet="false"
      v-model="zoom"
      v-model:zoom="zoom"
      :center="[48.1380244, 11.5835979]">
      <l-tile-layer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"></l-tile-layer>

      <l-circle :lat-lng="[48.1380244, 11.5835979]" :radius="500" color="rgba(255,0,0,0.3)" />

      <!-- <l-circle-marker :lat-lng="[41.89026, 12.49238]" :radius="0.01" /> -->
    </l-map>
  </div>
</template>

<script lang="ts">
import {
  LMap,
  LIcon,
  LTileLayer,
  LMarker,
  LControlLayers,
  LTooltip,
  LPopup,
  LCircleMarker,
  LCircle,
  LPolyline,
  LPolygon,
  LRectangle,
} from "@vue-leaflet/vue-leaflet";
import "leaflet/dist/leaflet.css";

export default {
  components: {
    LMap,
    LIcon,
    LTileLayer,
    LMarker,
    LControlLayers,
    LTooltip,
    LPopup,
    LCircleMarker,
    LPolyline,
    LPolygon,
    LCircle,
    LRectangle,
  },
  data() {
    return {
      zoom: 11,
      iconWidth: 25,
      iconHeight: 40,
    };
  },
  computed: {
    iconUrl() {
      return `https://placekitten.com/${this.iconWidth}/${this.iconHeight}`;
    },
    iconSize() {
      return [this.iconWidth, this.iconHeight];
    },
  },
  methods: {
    log(a: any) {
      console.log(a);
    },
    changeIcon() {
      this.iconWidth += 2;
      if (this.iconWidth > this.iconHeight) {
        this.iconWidth = Math.floor(this.iconHeight / 2);
      }
    },
  },
};
</script>
