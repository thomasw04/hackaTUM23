<script setup lang="ts">
import Map from './components/Map.vue'
</script>

<template>
  <nav class="navbar">
    <div class="navbar-brand">
      <a class="home-link navbar-item" href="/">
        <img class="logo-image" src="/vite.svg" alt="Check24 Challenge thingy">
      </a>
    </div>

    <div class="navbar-menu">
      <div class="navbar-start">
        <a class="navbar-item" href="/songs">
          <span class="bd-emoji">ඞ</span> &nbsp;Amongs
        </a>
      </div>

      <span class="navbar-item">
        <progress id="main-progress" class="progress is-small" style="display: none;"></progress>
      </span>
    </div>
  </nav>

  <router-view></router-view>
</template>

<style>
@import "../node_modules/bulma/css/bulma.min.css";
@import "../node_modules/bulma-prefers-dark/css/bulma-prefers-dark.min.css";

.navbar {
  position: sticky;
  width: 100%;
  height: 3%;
  top: 0;
  filter: drop-shadow(0 0 .25rem var(--navbar-drop-shadow));
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}

@media (prefers-color-scheme: dark) {
  a.navbar-item:focus, a.navbar-item:focus-within, a.navbar-item:hover {
      background-color: #2e2e2e !important;
      color: #aecdff !important;
  }
}
</style>
