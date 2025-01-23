<template>
    <div>
      <h1>Redirecting...</h1>
    </div>
  </template>
  
  <script>
  import { defineComponent } from 'vue';
  import { invoke } from '@tauri-apps/api/core';
  
  export default defineComponent({
    mounted() {
      const urlParams = new URLSearchParams(window.location.search);
      const authCode = urlParams.get('code'); // Retrieve the code from URL
  
      if (authCode) {
        // Call the Tauri backend to exchange the code for a token
        this.exchangeToken(authCode);
      }
    },
    methods: {
      async exchangeToken() {
        try {
          const response = await invoke<string>('exchange_spotify_token', { code });
          console.log('Token exchange successful:', response);
          // Here, you can store the token or navigate to another page
          this.$router.push({ name: 'home' }); // Redirect to the home page or wherever needed
        } catch (error) {
          console.error('Error during token exchange:', error);
        }
      }
    }
  });
</script>  