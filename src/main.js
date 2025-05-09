import { createApp } from 'vue';
import App from './App.vue';
import BootstrapVue3 from 'bootstrap-vue-3';  // Note: BootstrapVue 3 is for Vue 3
import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap-vue-3/dist/bootstrap-vue-3.css';  // Ensure you're using the correct package for Vue 3
import 'bootstrap-icons/font/bootstrap-icons.css';
import router from './router';

// Create Vue app instance
const app = createApp(App);

// Use BootstrapVue3 plugin
app.use(BootstrapVue3);

// Use Vue Router
app.use(router);

// Mount the app
app.mount('#app');
