<template>
  <b-container fluid class="vh-100 d-flex align-items-center justify-content-center bg-light">
    <b-row
      class="shadow-lg rounded overflow-hidden w-100"
      style="max-width: 900px; min-height: 500px;"
    >
      <!-- Left Gradient Panel -->
      <b-col
        md="5"
        class="d-none d-md-flex flex-column justify-content-center align-items-center text-white p-4"
        style="background: linear-gradient(135deg, #1F1C2C, #928DAB);"
      >
        <div class="text-center px-3">
          <h3 class="fw-bold mb-3">Welcome to Tauri Admin</h3>
          <p class="small">
            Manage users with blazing speed using Rust + Vue. Secure, fast, and elegant UI with modern design.
          </p>
        </div>
      </b-col>

      <!-- Create User Form -->
      <b-col cols="12" md="7" class="bg-white p-5 d-flex align-items-center">
        <div class="w-100">
          <h4 class="fw-semibold text-center mb-4">Create New User</h4>

          <b-form @submit.prevent="submitForm">
            <b-form-group label="Full Name" label-for="name" class="mb-3">
              <b-form-input
                id="name"
                v-model="form.name"
                placeholder="Enter full name"
                class="rounded-pill py-2 px-3 shadow-sm"
                required
              />
            </b-form-group>

            <b-form-group label="Email Address" label-for="email" class="mb-3">
              <b-form-input
                id="email"
                type="email"
                v-model="form.email"
                placeholder="Enter email address"
                class="rounded-pill py-2 px-3 shadow-sm"
                required
              />
            </b-form-group>

            <b-form-group label="User Role" label-for="role" class="mb-4">
              <b-form-select
                id="role"
                v-model="form.role"
                :options="roles"
                class="rounded-pill py-2 px-3 shadow-sm"
                required
              />
            </b-form-group>

            <b-form-group label="Password" label-for="password" class="mb-4">
              <b-form-input
                id="password"
                type="password"
                v-model="form.password"
                placeholder="Enter password"
                class="rounded-pill py-2 px-3 shadow-sm"
                required
              />
            </b-form-group>

            <b-button
              :disabled="loading"
              type="submit"
              class="w-100 rounded-pill py-2 fw-bold text-white"
              style="background: linear-gradient(to right, #6a11cb, #2575fc); border: none;"
            >
              {{ loading ? 'Creating...' : 'CREATE USER' }}
            </b-button>
          </b-form>
        </div>
      </b-col>
    </b-row>
  </b-container>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import Swal from 'sweetalert2';

export default {
  name: 'CreateUser',
  data() {
    return {
      form: {
        name: '',
        email: '',
        role: '',
        password: '',
      },
      roles: ['admin', 'user', 'manager'],
      loading: false,
    };
  },
  methods: {
    async submitForm() {
      this.loading = true;

      try {
        const token = localStorage.getItem('auth_token');
        if (!token) throw new Error('No authorization token found. Please login again.');

        await invoke('create_user_tauri', {
          user: this.form,
          token,
        });

        Swal.fire({
          icon: 'success',
          title: 'User created successfully!',
          showConfirmButton: false,
          timer: 1500,
        });

        this.form = { name: '', email: '', role: '', password: '' };

        setTimeout(() => {
          this.$router.push({ name: 'Read' });
        }, 1700);
      } catch (error) {
        Swal.fire({
          icon: 'error',
          title: 'Error',
          text: typeof error === 'string' ? error : (error.message || 'Failed to create user.'),
        });
      } finally {
        this.loading = false;
      }
    },
  },
};
</script>

<style scoped>
body {
  margin: 0;
  font-family: 'Segoe UI', sans-serif;
}

label {
  font-weight: 600;
  font-size: 0.95rem;
}
</style>
