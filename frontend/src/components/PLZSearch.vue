<template>
  <div class="search-page">
    <div>
      <progress v-if="isLoadingFinalResults" class="progress is-large"></progress>
      <p v-if="finalResults.length > 0">
        Showing {{ finalResults.length }} results for <b>{{ queryPLZ }}</b>
      </p>
    </div>

    <article class="message is-danger" v-if="finalResults.length <= 0 && !isLoadingFinalResults">
      <div class="message-header">
        <p>No results</p>
      </div>
      <div class="message-body">
        No results could be found for your search query. Go back to the
        <a class="has-text-weight-bold" href="/">home page</a> and try again.
      </div>
    </article>

    <div class="card" style="margin: 1em 0; padding: 0" v-for="provider in finalResults" :key="provider.id">
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

export default defineComponent({
  data() {
    return {
      queryPLZ: "",
      finalResults: [] as Array<ServiceProvider>,
      isLoadingFinalResults: false,
    };
  },
  mounted() {
    // Get query parameter from the router/URL
    this.queryPLZ = this.$route.query.q?.toString() ?? "";
    this.loadResults();
  },
  methods: {
    async loadResults() {
      console.log("Loading results for query:", this.queryPLZ);
      let queryCopy = this.queryPLZ;
      this.isLoadingFinalResults = true;

      setTimeout(() => {
        this.finalResults = Array(Math.floor(Math.random() * 15)).fill(alphonso);
        this.isLoadingFinalResults = false;
        this.$router.push({ query: { q: queryCopy } });
      }, Math.random() * 1250);

      return;

      try {
        // TODO: actually fetch craftsmen
        let response = await fetch(`/zipcode/search?q=${this.queryPLZ}`).then((response) => response.json());
        // Assuming the response data is an array of ServiceProvider objects
        this.finalResults = response;
        this.$router.push({ query: { q: queryCopy } });
      } catch (e: unknown) {
        console.log("Final results error:", e);
      } finally {
        this.isLoadingFinalResults = false;
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
