import './assets/main.css';

import { createApp } from 'vue';
import { createPinia } from 'pinia';

import App from './App.vue';
import router from './router';
import { createPersistedState } from 'pinia-plugin-persistedstate';

const app = createApp(App);

const pinia = createPinia();
pinia.use(createPersistedState({
  auto: true,
}));

app.use(pinia);
app.use(router);

app.mount('#app');
