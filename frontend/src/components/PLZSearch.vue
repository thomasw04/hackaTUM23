<template>
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
      :search-p-l-z-coords="[48.249, 11.651]"
    />

    <div class="mt-2 columns is-2">
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
        Showing {{ results.length }} out of {{ totalCount }} results for <b>{{ queryPLZ }}</b>
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

@media (prefers-color-scheme: dark) {
  .is-active {
    background-color: #4f4f4f !important;
    color: #aecdff !important;
  }
}
</style>
