// src/router/index.js
import { createRouter, createWebHistory } from 'vue-router';

import LoginPage from '@views/Login.vue';
import Register from '@views/Register.vue';
import CreateUser from '@pages/Create.vue';
import ReadUsers from '@pages/Read.vue';
import UpdateUser from '@pages/Update.vue';
import DeleteUser from '@pages/Delete.vue';
import Dashboard from '@layouts/Dashboard.vue';
import AuthenticatedLayout from '@layouts/AuthenticatedLayout.vue';


const routes = [
  // Public routes
  { path: '/login', component: LoginPage, name: 'Login' },
  { path: '/register', component: Register, name: 'Register' },

  // Authenticated routes wrapped in layout
  {
    path: '/',
    component: AuthenticatedLayout,
    meta: { requiresAuth: true },
    children: [
      { path: '', redirect: { name: 'Read' } },
      { path: 'dashboard', component: Dashboard, name: 'Dashboard' },
      { path: '/read', component: ReadUsers, name: 'Read' },
      { path: '/create', component: CreateUser, name: 'Create' },
      { path: '/update/:id', component: UpdateUser, name: 'Update', props: true },
      { path: '/delete/:id', component: DeleteUser, name: 'Delete', props: true },

    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

// Navigation Guard
router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('auth_token');
  if (to.meta.requiresAuth && !token) {
    next({ name: 'Login' });
  } else {
    next();
  }
});

export default router;
