import { createApp } from "vue";
import App from "./App.vue";
import { createRouter, createWebHashHistory } from "vue-router";
import CallbackPage from "./CallbackPage.vue";
import Login from "./Login.vue";

const routes = [
  { path: '/', component: App },
  { path: '/callback', component: CallbackPage }, // Route for handling Spotify callback
  { path: '/login', component: Login },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

createApp(App).use(router).mount("#app");