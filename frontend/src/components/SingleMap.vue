<template>
  <l-map :use-global-leaflet="false"
    v-model="zoomLvl"
    v-model:zoom="zoomLvl"
    :center="coords">

    <l-tile-layer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"></l-tile-layer>

    <l-circle-marker v-if="radius" :lat-lng="coords" :radius="radius" color="rgba(255,0,0,0.3)" />
    <l-marker :lat-lng="coords" />
  </l-map>
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
  props: {
    coords: {
      type: Array,
      required: true,
    },
    zoom: {
      type: Number,
      default: 10,
    },
    radius: {
      type: Number,
    }
  },
  data() {
    return {
      zoomLvl: this.$props.zoom,
    };
  }
};
</script>
