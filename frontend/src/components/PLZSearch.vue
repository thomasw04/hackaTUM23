<template>
  <div class="modal is-active" v-if="editServiceProvider && editItems">
    <div class="modal-background"></div>
    <div class="modal-card">
      <header class="modal-card-head">
        <p class="modal-card-title">Edit Service Provider</p>
        <button class="delete" aria-label="close" @click="editServiceProvider = null"></button>
      </header>
      <section class="modal-card-body">
        <div class="field">
          <label class="label">Max Driving Distance</label>
          <div class="control">
            <input
              class="input"
              type="number"
              v-model="editItems.maxDrivingDistance"
              placeholder="Enter max driving distance"
            />
          </div>
        </div>
        <div class="field">
          <label class="label">Profile Picture Score</label>
          <div class="control">
            <input
              class="input"
              type="number"
              v-model="editItems.profilePictureScore"
              placeholder="Enter profile picture score"
            />
          </div>
        </div>
        <div class="field">
          <label class="label">Profile Description Score</label>
          <div class="control">
            <input
              class="input"
              type="number"
              v-model="editItems.profileDescriptionScore"
              placeholder="Enter profile description score"
            />
          </div>
        </div>
        <div v-if="editServiceProviderError">
          <p class="always-light has-text-danger">{{ editServiceProviderError }}</p>
          <br />
        </div>
      </section>
      <footer class="modal-card-foot">
        <button
          class="button is-success"
          :class="{ 'is-loading': editServiceProviderLoading }"
          @click="saveServiceProvider"
        >
          Save
        </button>
        <button class="button" @click="editServiceProvider = null">Cancel</button>
      </footer>
    </div>
  </div>

  <div class="search-page">
    <article class="message is-danger" v-if="results.length <= 0 && !isLoadingResults">
      <div class="message-header">
        <p>No results</p>
      </div>
      <div class="message-body">
        No results could be found for your search query. Go back to the
        <a class="has-text-weight-bold" href="/">home page</a> and try again.
      </div>
    </article>

    <ServiceProviderMap
      :service-providers="results"
      style="position: relative; min-height: 50vh"
      :search-p-l-z-coords="mapCoords"
      :edit-service-provider-func="openEditDialog"
    />

    <div class="custom-outline mt-2 columns is-2">
      <div class="column">
        <div class="select is-fullwidth" @change="setRankType">
          <select>
            <option value="rank">Rank</option>
            <option value="distance">Distance</option>
            <option value="profile">Profile</option>
          </select>
        </div>
      </div>
      <div class="column">
        <button
          style="background-color: #0271c2"
          class="button is-primary is-fullwidth"
          @click="loadResults"
          :disabled="isLoadingResults || !haveMoreResults"
          :class="{ 'is-loading': isLoadingResults }"
        >
          Load more
        </button>
      </div>
    </div>

    <div class="mt-2">
      <p v-if="results.length > 0">
        Showing {{ results.length }} out of {{ totalCount }} results for
        <b
          >{{ queryPLZ }}<template v-if="centerCityName">&nbsp;{{ centerCityName }}</template></b
        >
      </p>
    </div>

    <div class="card" style="margin: 1em 0; padding: 0" v-for="provider in results" :key="provider.id">
      <header class="card-header">
        <p class="card-header-title">{{ provider.first_name }} {{ provider.last_name }}</p>
        <button class="card-header-icon" aria-label="more options">
          <span class="icon">
            <i class="fas fa-angle-down" aria-hidden="true"></i>
          </span>
        </button>
      </header>
      <div class="card-content">
        <div class="content">
          <b>{{ provider.city }}</b
          >, {{ provider.street }} {{ provider.house_number }}
          <br />
          <br />
          <i
            >{{ provider.first_name }} is ready to drive up to
            {{ Math.floor(provider.max_driving_distance / 1000) }}km</i
          >
          <br />
        </div>
      </div>
      <footer class="card-footer">
        <a @click.prevent href="#" class="card-footer-item">Book</a>
        <a @click.prevent href="#" class="card-footer-item">Message</a>
        <a @click.prevent href="#" class="card-footer-item">Remember</a>
      </footer>
    </div>

    <div v-if="haveMoreResults">
      <button class="button is-primary is-fullwidth" @click="loadResults" :class="{ 'is-loading': isLoadingResults }">
        Load more
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ServiceProvider } from "../models/results";
import ServiceProviderMap from "./ServiceProviderMap.vue";

interface ServiceProviderResponse {
  results: Array<ServiceProvider>;
  has_more: boolean;
  total_count: number;
  postcode_info: {
    zipcode: number;
    place: string;
    latitude: number;
    longitude: number;
  } | null;
}

export default defineComponent({
  data() {
    return {
      page: 0,
      queryPLZ: "",
      rankType: "rank" as "rank" | "distance" | "profile",
      results: [] as Array<ServiceProvider>,
      isLoadingResults: false,

      haveMoreResults: false,
      totalCount: 0,

      mapCoords: [48.249, 11.651],
      centerCityName: "",

      editServiceProvider: null as ServiceProvider | null,
      editServiceProviderLoading: false,
      editServiceProviderError: null as string | null,
      editItems: null as {
        maxDrivingDistance: number | null;
        profilePictureScore: number | null;
        profileDescriptionScore: number | null;
      } | null,
    };
  },
  components: {
    ServiceProviderMap,
  },
  mounted() {
    // Get query parameter from the router/URL
    this.queryPLZ = this.$route.query.q?.toString() ?? "";
    this.loadResults();
  },
  methods: {
    openEditDialog(sp: ServiceProvider) {
      this.editServiceProvider = sp;
      this.editItems = {
        maxDrivingDistance: sp.max_driving_distance,
        profilePictureScore: null,
        profileDescriptionScore: null,
      };
    },
    closeEditDialog() {
      this.editServiceProvider = null;
      this.editItems = null;
    },
    async saveServiceProvider() {
      if (!this.editServiceProvider || !this.editItems) {
        throw new Error("editServiceProvider or editItems is not set");
      }
      this.editServiceProviderLoading = true;
      this.editServiceProviderError = null;

      try {
        let response = await fetch(`/craftman/${this.editServiceProvider.id}`, {
          method: "PATCH",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(this.editItems),
        });
        if (!response.ok) {
          throw new Error("Server returned not OK while saving the service provider");
        }

        this.closeEditDialog();
      } catch (e: unknown) {
        console.log("Edit error:", e);
        this.editServiceProviderError = String(e);
      } finally {
        this.editServiceProviderLoading = false;
      }
    },

    async setRankType(event: Event) {
      let target = event.target as HTMLSelectElement;
      this.rankType = target.value as "rank" | "distance" | "profile";
      this.page = 0;
      this.results = [];
      this.loadResults();
    },
    async fetchCraftsmen(page?: number): Promise<ServiceProviderResponse> {
      return fetch(`/craftsmen/${this.queryPLZ}/detailed?sort=${this.rankType}&page=${page ?? 0}`).then((response) =>
        response.json(),
      );
    },
    async loadResults() {
      console.log("Loading results for query:", this.queryPLZ);
      let queryCopy = this.queryPLZ;
      this.isLoadingResults = true;

      try {
        let currentResults = await this.fetchCraftsmen(this.page);
        if (this.page === 0) {
          this.results = currentResults.results;
          this.$router.push({ query: { q: queryCopy } });
        } else {
          this.results = this.results.concat(currentResults.results);
        }
        this.haveMoreResults = currentResults.has_more;
        this.totalCount = currentResults.total_count;
        if (this.haveMoreResults) {
          this.page++;
        }
        let coords = currentResults.postcode_info;
        this.mapCoords = coords ? [coords.latitude, coords.longitude] : [48.249, 11.651];
        this.centerCityName = coords?.place.toString() ?? "";
      } catch (e: unknown) {
        console.log("Results fetching error:", e);
      } finally {
        this.isLoadingResults = false;
      }
    },
  },
});
</script>

<style scoped>
.search-page {
  margin: 0 auto;
  padding: 20px;
}

.custom-outline {
  background-color: #f8b11e;
  padding: 1px;
  border: 1px solid #f8b11e;
  border-radius: 8px;
}

.search-results {
  list-style-type: none;
  padding: 0;
  margin-top: 20px;
}

.search-results li {
  margin-bottom: 10px;
}

.is-active {
  background-color: lightgray;
}

#search-suggestions {
  border: solid 1px #ccc;
  border-top: none;
}

@media (prefers-color-scheme: dark) {
  .is-active {
    background-color: #4f4f4f !important;
    color: #aecdff !important;
  }
}
</style>
