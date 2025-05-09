<template>
  <b-container fluid class="d-flex align-items-center justify-content-center bg-light p-3" style="min-height: 100vh;">
    <b-row
      class="shadow-lg rounded w-100"
      style="max-width: 600px; background-color: #fff;"
    >
      <b-col cols="12" class="p-4">
        <h4 class="text-center mb-4 text-primary">Register</h4>

        <!-- Alert -->
        <b-alert :show="alert.show" :variant="alert.variant" dismissible @dismissed="alert.show = false">
          {{ alert.message }}
        </b-alert>

        <b-form @submit.prevent="registerUser">
          <b-form-group label="Name" label-for="name">
            <b-form-input
              id="name"
              v-model="user.name"
              placeholder="Enter full name"
              class="rounded-pill py-2 px-3 shadow-sm"
              :state="getFieldState(user.name)"
              required
            />
            <b-form-invalid-feedback v-if="!user.name">Name is required.</b-form-invalid-feedback>
          </b-form-group>

          <b-form-group label="Email" label-for="email">
            <b-form-input
              id="email"
              type="email"
              v-model="user.email"
              placeholder="Enter email address"
              class="rounded-pill py-2 px-3 shadow-sm"
              :state="getFieldState(user.email)"
              required
            />
            <b-form-invalid-feedback v-if="!user.email">Email is required.</b-form-invalid-feedback>
          </b-form-group>

          <b-form-group label="Password" label-for="password">
            <b-form-input
              id="password"
              type="password"
              v-model="user.password"
              placeholder="Enter password"
              class="rounded-pill py-2 px-3 shadow-sm"
              :state="getFieldState(user.password)"
              required
            />
            <b-form-invalid-feedback v-if="!user.password">Password is required.</b-form-invalid-feedback>
          </b-form-group>

          <b-form-group label="Role" label-for="role">
            <b-form-select
              id="role"
              v-model="user.role"
              :options="roles"
              class="rounded-pill py-2 px-3 shadow-sm"
              :state="getFieldState(user.role)"
              required
            />
            <b-form-invalid-feedback v-if="!user.role">Please select a role.</b-form-invalid-feedback>
          </b-form-group>

          <b-row class="mt-4">
            <b-col cols="12" class="d-flex flex-column flex-md-row justify-content-between gap-2">
              <b-button
                variant="danger"
                class="rounded-pill py-2 px-4 fw-bold w-100"
                @click="$router.push('/')"
              >
                Back to Login
              </b-button>

              <b-button
                type="submit"
                class="rounded-pill py-2 px-4 fw-bold w-100 text-white"
                style="background: linear-gradient(to right, #6a11cb, #2575fc); border: none;"
              >
                Register
              </b-button>
            </b-col>
          </b-row>
        </b-form>
      </b-col>
    </b-row>
  </b-container>
</template>

<script>
import Swal from 'sweetalert2';
import { invoke } from '@tauri-apps/api/core';

export default {
  data() {
    return {
      user: { name: '', email: '', password: '', role: '' },
      alert: { show: false, variant: '', message: '' },
      roles: ['admin', 'user']
    };
  },
  methods: {
  getFieldState(field) {
      return field ? true : null;
    },
    async registerUser() {
  console.log("Registering user with data:", this.user);

  if (!this.user.name || !this.user.email || !this.user.password || !this.user.role) {
    Swal.fire({
      icon: 'error',
      title: 'Missing Fields',
      text: 'Please fill in all required fields.'
    });
    return;
  }

  try {
    const result = await invoke('register_tauri', { user: this.user });
    console.log("Tauri response:", result);

    Swal.fire({
      icon: 'success',
      title: 'Registered!',
      text: 'User registered successfully.'
    });

    this.user = { name: '', email: '', password: '', role: '' };
  } catch (err) {
    console.error("Registration error:", err);
    Swal.fire({
      icon: 'error',
      title: 'Registration Failed',
      text: 'Error registering user. Please try again.'
    });
  }
}
  }
};
</script>

<style scoped>
@media (max-width: 768px) {
  .gap-2 {
    gap: 0.75rem;
  }
}
</style>
