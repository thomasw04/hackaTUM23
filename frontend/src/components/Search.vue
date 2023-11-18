<template>
  <div class="search-page">
    <div class="field">
      <label class="label">Search:</label>
      <form id="search-form" action="/search" @submit.prevent>
        <div id="search-container" class="control has-icons-left has-icons-right is-large" :class="{'is-loading': isLoadingAutocomplete}">
          <input autofocus id="search-bar" v-model="searchQuery" class="input is-large" type="text" placeholder="Postal code" autocomplete="off" @input="loadAutocomplete" @keydown.down.prevent="handleArrowDown" @keydown.up.prevent="handleArrowUp" @keydown.enter="handleEnter" />
          <span class="icon is-small is-left">🔍</span>
          <span class="icon is-small is-right">{{ }}</span>
        </div>

        <div id="search-suggestions" v-if="showAutocomplete">
          <ul>
            <li v-for="(result, index) in autocompleteResults" :key="result.zipcode" :class="{ 'is-active': index === activeAutocompleteIndex }">
              <a class="navbar-item" @click="selectZipcode(result.zipcode)">
                {{ result.zipcode }} {{ result.place }}
              </a>
            </li>
          </ul>
        </div>
      </form>
    </div>

    <div>
      <progress v-if="isLoadingFinalResults" class="progress is-large is-info"></progress>
      <p v-if="finalResults.length > 0">
        Showing {{ finalResults.length }} results for <b>{{ finalResultsFor }}</b>
      </p>
    </div>

    <article class="message is-danger" v-if="finalResults.length <= 0 && !isLoadingFinalResults">
      <div class="message-header">
        <p>No results</p>
      </div>
      <div class="message-body">
        No results could be found for your search query.
      </div>
    </article>

    <div class="card" style="margin: 1em 0; padding: 0" v-for="provider in finalResults" :key="provider.id">
      <header class="card-header">
        <p class="card-header-title">
          {{ provider.first_name }} {{ provider.last_name }}
        </p>
        <button class="card-header-icon" aria-label="more options">
          <span class="icon">
            <i class="fas fa-angle-down" aria-hidden="true"></i>
          </span>
        </button>
      </header>
      <div class="card-content">
        <div class="content">
          <b>{{ provider.city }}</b>,
          {{ provider.street }} {{ provider.house_number }}
          <br>
          <i>{{ provider.first_name }} is ready to drive up to {{Math.floor(provider.max_driving_distance / 1000)}}km</i>
          <br>
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
import { defineComponent } from 'vue';
import Map from './Map.vue';

interface ZipcodeSearchResultItem {
  zipcode: number;
  place: string;
  latitude: number;
  longitude: number;
}

interface ServiceProvider {
  id: number;
  first_name: string;
  last_name: string;
  city: string;
  street: string;
  house_number: string;
  lon: number;
  lat: number;
  max_driving_distance: number;
}


var alphonso: ServiceProvider = {
  "id": 64210,
  "first_name": "Alphonso",
  "last_name": "OConner",
  "city": "New Leifburgh",
  "street": "Quitzon Brook",
  "house_number": "3",
  "lon": 9.753894,
  "lat": 48.680219,
  "max_driving_distance": 63000
}

export default defineComponent({
  data() {
    return {
      searchQuery: '85748', // just a default value
      autocompleteResults: [] as Array<ZipcodeSearchResultItem>,
      activeAutocompleteIndex: -1,
      isLoadingAutocomplete: false,
      showAutocomplete: false,
      provider: alphonso,

      finalResults: [] as Array<ServiceProvider>,
      finalResultsFor: '',
      isLoadingFinalResults: false,
    };
  },
  components: {
    Map,
  },
  mounted() {
    // Get query parameter from the router/URL
    this.searchQuery = this.$route.query.q?.toString() ?? '';

    this.loadResults()
  },
  methods: {
    async loadAutocomplete() {
      this.isLoadingAutocomplete = true;
      this.showAutocomplete = true;

      try {

        let response = await fetch(`/zipcode/search?q=${this.searchQuery}`)
          .then(response => response.json());
        // Assuming the response data is an array of Craftsman objects
        this.autocompleteResults = response;
        this.activeAutocompleteIndex = -1;
      } catch (e: any) {
        console.log("Autocomplete error:", e)
      } finally {
        this.isLoadingAutocomplete = false;
      }
    },
    async loadResults() {
      console.log("Loading results for query:", this.searchQuery);
      let queryCopy = this.searchQuery;
      this.isLoadingFinalResults = true;

      setTimeout(() => {
        this.finalResults = Array(Math.floor(Math.random() * 15)).fill(alphonso);
        this.finalResultsFor = queryCopy;
        this.isLoadingFinalResults = false;
        this.showAutocomplete = false;
        this.$router.push({ query: { q: queryCopy } });
      }, Math.random() * 1250);

      return;

      try {
        // TODO: actually fetch craftsmen
        let response = await fetch(`/zipcode/search?q=${this.searchQuery}`)
          .then(response => response.json());
        // Assuming the response data is an array of ServiceProvider objects
        this.finalResults = response;
        this.finalResultsFor = queryCopy;
        this.showAutocomplete = false;
        this.$router.push({ query: { q: queryCopy } });
      } catch (e: any) {
        console.log("Final results error:", e)
      } finally {
        this.isLoadingFinalResults = false;
      }
    },
    selectZipcode(code: number) {
      this.activeAutocompleteIndex = -1;
      this.searchQuery = code.toString();
      this.loadResults();
    },
    handleArrowDown() {
      if (this.activeAutocompleteIndex < this.autocompleteResults.length - 1) {
        this.activeAutocompleteIndex++;
      }
    },
    handleArrowUp() {
      if (this.activeAutocompleteIndex >= 0) {
        this.activeAutocompleteIndex--;
      }
    },
    handleEnter() {
      if (this.activeAutocompleteIndex >= 0) {
        this.selectZipcode(this.autocompleteResults[this.activeAutocompleteIndex].zipcode);
      }
      this.loadResults();
    },
  },
});
</script>

<style scoped>
.search-page {
  max-width: 600px;
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
  border: solid 1px #CCC;
  border-top: none;
}

@media (prefers-color-scheme: dark) {
  .is-active {
    background-color: #4f4f4f !important;
    color: #aecdff !important;
  }
}
</style>
