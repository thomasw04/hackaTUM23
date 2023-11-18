<template>
  <l-map :use-global-leaflet="false" v-model="zoomLvl" v-model:zoom="zoomLvl" :center="searchPLZCoords as any">
    <l-tile-layer url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"></l-tile-layer>

    <template v-if="searchPLZCoords">
      <l-circle-marker
        v-if="searchRadius"
        :lat-lng="searchPLZCoords as any"
        :radius="searchRadius"
        color="rgba(255,0,0,0.3)"
      />
      <l-marker :lat-lng="searchPLZCoords as any" color="rgba(255,0,0,1)" />
    </template>

    <l-marker
      v-for="sp in serviceProviders"
      :key="sp.id"
      :lat-lng="getNormalCoords(sp)"
      :title="sp.last_name"
      :alt="sp.last_name"
    >
      <l-popup>
        <h3>{{ sp.first_name }} {{ sp.last_name }}</h3>
        <p class="always-light" style="margin: 0">
          <b>{{ sp.city }}</b>
        </p>
        <p class="always-light" style="margin: 0">{{ sp.street }} {{ sp.house_number }}</p>

        <button v-if="editServiceProviderFunc" class="button is-small is-primary" @click="editServiceProvider(sp)">
          Edit
        </button>
      </l-popup>
    </l-marker>
  </l-map>
</template>

<script lang="ts">
import { LMap, LTileLayer, LMarker, LCircleMarker, LPopup } from "@vue-leaflet/vue-leaflet";
import "leaflet/dist/leaflet.css";
import { ServiceProvider, getNormalCoords } from "../models/results";

export default {
  components: {
    LMap,
    LTileLayer,
    LMarker,
    LPopup,
    LCircleMarker,
  },
  props: {
    serviceProviders: {
      type: Array<ServiceProvider>,
      required: true,
    },
    searchPLZCoords: {
      type: Array<number>,
      required: true,
    },
    searchRadius: {
      type: Number,
    },
    editServiceProviderFunc: {
      type: Function as unknown as () => (sp: ServiceProvider) => void,
      required: true,
    },
  },
  data() {
    return {
      zoomLvl: 11,
    };
  },
  methods: {
    getNormalCoords,
    editServiceProvider(sp: ServiceProvider) {
      if (!this.editServiceProviderFunc) {
        throw new Error("editServiceProvider is not set");
      }
      this.editServiceProviderFunc(sp);
    },
  },
};
</script>
