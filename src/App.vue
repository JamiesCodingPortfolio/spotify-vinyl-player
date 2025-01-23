<template>
  <div class="relative h-screen flex items-center justify-center">
    <!-- Left Button -->
    <button 
      class="absolute left-4 px-4 py-2 text-lg font-medium text-white bg-blue-500 rounded-md disabled:bg-gray-300 disabled:cursor-not-allowed hover:bg-blue-700 transition" 
      @click="goLeft" 
      :disabled="currentIndex === 0"
    >
      &#8592;
    </button>

    <!-- Card Container -->
    <div class="flex items-center justify-center overflow-hidden" style="width: 35vmin; height: 35vmin;">
      <div 
        v-for="(card, index) in cards" 
        :key="index" 
        class="absolute flex items-center justify-center bg-gray-100 border border-gray-300 rounded-lg transition-transform transform"
        :class="{
          'opacity-100 scale-100 z-10': index === currentIndex,
          'opacity-50 scale-75 z-0': index !== currentIndex
        }"
        style="width: 35vmin; height: 35vmin;"
      >
        {{ card }}
      </div>
    </div>

    <!-- Right Button -->
    <button 
      class="absolute right-4 px-4 py-2 text-lg font-medium text-white bg-blue-500 rounded-md disabled:bg-gray-300 disabled:cursor-not-allowed hover:bg-blue-700 transition" 
      @click="goRight" 
      :disabled="currentIndex === cards.length - 1"
    >
      &#8594;
    </button>

    <!-- Spotify Login Button -->
    <button
      class="absolute bottom-4 px-6 py-3 text-lg font-medium text-white bg-green-500 rounded-md hover:bg-green-700 transition"
      @click="loginWithSpotify"
    >
      Log into Spotify
    </button>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export default defineComponent({
  name: 'CardCarousel',
  setup() {
    // Define a list of cards
    const cards = ref<string[]>(['Card 1', 'Card 2', 'Card 3', 'Card 4']);

    // Track the current active card index
    const currentIndex = ref<number>(0);

    // Function to move left
    const goLeft = () => {
      if (currentIndex.value > 0) {
        currentIndex.value--;
      }
    };

    // Function to move right
    const goRight = () => {
      if (currentIndex.value < cards.value.length - 1) {
        currentIndex.value++;
      }
    };

    // Function to log in to Spotify
    const loginWithSpotify = async () => {
      // Get the Spotify authorization URL from the backend
      const authUrl = await invoke<string>('generate_spotify_auth_url');
      window.location.href = authUrl; // Redirect user to Spotify login
    };

    return {
      cards,
      currentIndex,
      goLeft,
      goRight,
      loginWithSpotify,
    };
  },
});
</script>

<style scoped>
/* Tailwind CSS is used for styling; no additional custom CSS needed. */
</style>
