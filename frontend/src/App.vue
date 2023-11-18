<script setup lang="ts">
import Map from './components/Map.vue'
</script>

<template>
  <nav class="navbar" style="background-color: #063773;">
    <div class="navbar-brand">
      <a class="home-link navbar-item" href="/">
        <img class="logo-image" src="/vite.svg" alt="Check24 Challenge thingy">
          <h1 class="navbar-item" style="font-style: italic; font-size: 2.5em; font-weight: 900; color: #fff">
            Not Check24
          </h1>
        </a>
    </div>
  </nav>

  <router-view id="#content-container"></router-view>
</template>

<style>
@import "../node_modules/bulma/css/bulma.min.css";
@import "../node_modules/bulma-prefers-dark/css/bulma-prefers-dark.min.css";
@import "assets/wireframe.css";

body {
  font-family: Verdana, Arial, Helvetica, sans-serif;
  font-size: medium;
}

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

  p {
    color: #fff !important;
  }
}
</style>
