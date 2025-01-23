<template>
    <div class="h-screen flex items-center justify-center">
      <button 
        class="px-6 py-3 text-lg font-medium text-white bg-green-500 rounded-md hover:bg-green-700 transition"
        @click="loginWithSpotify"
      >
        Log into Spotify
      </button>
    </div>
  </template>
  
<script lang="ts">
  import { defineComponent } from 'vue';
  import { invoke } from '@tauri-apps/api/core';
  
  export default defineComponent({
    name: 'LoginPage',
    methods: {
      async loginWithSpotify() {
        try {
          // Call the Tauri backend to get the Spotify auth URL
          const authUrl = await invoke<string>('generate_spotify_auth_url');
          window.location.href = authUrl; // Redirect the user to Spotify login
        } catch (error) {
          console.error('Error during login:', error);
        }
      }
    }
  });
</script>
  
<style scoped>
  /* You can add additional styling here if needed, but Tailwind should handle most of the layout */
</style>
  