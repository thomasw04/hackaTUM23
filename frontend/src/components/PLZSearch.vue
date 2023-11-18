<template>
  <div class="search-page">
    <div>
      <progress v-if="isLoadingResults" class="progress is-large"></progress>
      <p v-if="results.length > 0">
        Showing {{ results.length }} results for <b>{{ queryPLZ }}</b>
      </p>
    </div>

    <article class="message is-danger" v-if="results.length <= 0 && !isLoadingResults">
      <div class="message-header">
        <p>No results</p>
      </div>
      <div class="message-body">
        No results could be found for your search query. Go back to the
        <a class="has-text-weight-bold" href="/">home page</a> and try again.
      </div>
    </article>

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
      <button class="button is-primary is-fullwidth" @click="loadResults"
      :class="{ 'is-loading': isLoadingResults }">Load more</button>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { ServiceProvider } from "../models/results";

var alphonso: ServiceProvider = {
  id: 64210,
  first_name: "Alphonso",
  last_name: "OConner",
  city: "New Leifburgh",
  street: "Quitzon Brook",
  house_number: "3",
  lon: 9.753894,
  lat: 48.680219,
  max_driving_distance: 63000,
};

interface ServiceProviderResponse {
  results: Array<ServiceProvider>;
  haveMoreResults: boolean;
}


export default defineComponent({
  data() {
    return {
      page: 0,
      queryPLZ: "",
      results: [] as Array<ServiceProvider>,
      isLoadingResults: false,

      haveMoreResults: false,
    };
  },
  mounted() {
    // Get query parameter from the router/URL
    this.queryPLZ = this.$route.query.q?.toString() ?? "";
    this.loadResults();
  },
  methods: {
    async fetchCraftsmen(page?: number): Promise<ServiceProviderResponse> {
      return new Promise((resolve, _) => {
        setTimeout(() => {
          let data = Array(20).fill(alphonso);
          resolve({
            haveMoreResults: page == 0 || Math.random() > 0.8,
            results: data,
          });
        }, Math.random() * 1500);
      });

      return fetch(`/zipcode/search?q=${this.queryPLZ}&page=${page ?? 0}`).then((response) => response.json());
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
        this.haveMoreResults = currentResults.haveMoreResults;
        if (this.haveMoreResults) {
            this.page++;
        }
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

.progress {
  background-image: linear-gradient(to right, #00d1b2 30%, #ededed 30%) !important;
}

@media (prefers-color-scheme: dark) {
  .is-active {
    background-color: #4f4f4f !important;
    color: #aecdff !important;
  }

  .progress {
    background-image: linear-gradient(to right, #00d1b2 30%, #363636 30%) !important;
  }
}
</style>