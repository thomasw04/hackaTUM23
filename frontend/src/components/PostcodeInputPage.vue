<template>
  <div class="search-page">

    <div style="position:relative; height:100%" v-if="previewCoords">
      <SingleMap style="position:relative; min-height: 300px;" :coords="previewCoords" />
    </div>

    <div class="field pt-2">
      <form id="search-form" action="/search" @submit.prevent>
        <div id="search-container" class="control has-icons-left has-icons-right is-large" :class="{ 'is-loading': isLoadingAutocomplete }">
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

        <div class="pt-2">
          <button class="button is-primary is-fullwidth" @click="handleEnter(true)">Search</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import SingleMap from './SingleMap.vue';
import { ZipcodeSearchResultItem } from '../models/autocomplete.ts';

export default defineComponent({
  data() {
    return {
      searchQuery: '85748', // just a default value
      autocompleteResults: [] as Array<ZipcodeSearchResultItem>,
      activeAutocompleteIndex: -1,
      isLoadingAutocomplete: false,
      showAutocomplete: false,
      previewCoords: [48.249, 11.651] as Array<number> | null,
    };
  },
  components: {
    SingleMap,
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
    selectZipcode(code: number) {
      this.activeAutocompleteIndex = -1;
      this.searchQuery = code.toString();
      this.showAutocomplete = false;
    },
    handleArrowDown() {
      if (this.activeAutocompleteIndex < this.autocompleteResults.length - 1) {
        this.activeAutocompleteIndex++;
      }

      this.setPreviewCoords();
    },
    handleArrowUp() {
      if (this.activeAutocompleteIndex >= 0) {
        this.activeAutocompleteIndex--;
      }
      this.setPreviewCoords();
    },
    handleEnter(force_router: boolean) {
      let zipCode = this.activeAutocompleteIndex >= 0 ? this.autocompleteResults[this.activeAutocompleteIndex].zipcode : parseInt(this.searchQuery);
      if (isNaN(zipCode)) {
        return;
      }

      if (force_router || this.activeAutocompleteIndex < 0) {
        this.$router.push(`/search?q=${zipCode}`);
      } else {
        this.setPreviewCoords();
        this.selectZipcode(zipCode);
      }
    },
    setPreviewCoords(index?: number) {
      let item = this.autocompleteResults[index ?? this.activeAutocompleteIndex];
      this.previewCoords = [
        item.latitude, item.longitude
      ]
    }
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
