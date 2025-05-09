<template>
  <b-container fluid class="login-bg d-flex justify-content-center align-items-center p-3">
    <div class="overlay w-100" style="max-width: 400px;">
      <h4 class="text-center mb-4 text-primary">Login</h4>

      <b-form @submit.prevent="handleLogin">
        <!-- Email -->
        <b-form-group label="Email" label-for="email">
          <b-form-input
            id="email"
            v-model="form.email"
            type="email"
            required
            placeholder="Enter email"
            class="rounded-pill"
          ></b-form-input>
        </b-form-group>

        <!-- Password -->
        <b-form-group label="Password" label-for="password">
          <b-form-input
            id="password"
            v-model="form.password"
            type="password"
            required
            placeholder="Enter password"
            class="rounded-pill"
          ></b-form-input>
        </b-form-group>

        <!-- Remember Me and Forgot Password -->
        <b-row class="mb-3">
          <b-col cols="6">
            <b-form-checkbox v-model="form.rememberMe">Remember me</b-form-checkbox>
          </b-col>
          <b-col cols="6" class="text-end">
            <b-button variant="link" class="p-0" @click="forgotPassword">Forgot password?</b-button>
          </b-col>
        </b-row>

        <!-- Login Button -->
        <b-button
          type="submit"
          class="w-100 rounded-pill py-2 fw-bold text-white"
          :disabled="loading"
          style="background: linear-gradient(to right, #6a11cb, #2575fc); border: none;"
        >
          {{ loading ? 'Logging in...' : 'Login' }}
        </b-button>

        <!-- Create Account Link -->
        <div class="text-center mt-3">
          <span>Don't have an account?</span>
          <b-button variant="link" class="p-0" @click="goToRegisterPage">Create Account</b-button>
        </div>
      </b-form>
    </div>
  </b-container>
</template>

<script>
import Swal from 'sweetalert2';
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'LoginPage',
  data() {
    return {
      form: {
        email: '',
        password: '',
        rememberMe: false,
      },
      loading: false,
    };
  },
  methods: {
    async handleLogin() {
      this.loading = true;

      try {
        const response = await invoke('login_tauri', { form: this.form });

        // Allow login only if user is an admin
        if (response && response.token && response.role?.toLowerCase() === 'admin') {
          localStorage.setItem('auth_token', response.token);
          localStorage.setItem('user_info', JSON.stringify({
            name: response.name || 'Admin',
            role: response.role,
          }));

          await Swal.fire({
            icon: 'success',
            title: 'Login Successful',
            text: `Welcome, ${response.name || 'Admin'}!`,
            timer: 1500,
            showConfirmButton: false,
          });

          this.$router.push('/');
        } else {
          await Swal.fire({
            icon: 'error',
            title: 'Access Denied',
            text: 'Only users with the "admin" role can log in.',
          });
        }
      } catch (err) {
        await Swal.fire({
          icon: 'error',
          title: 'Login Failed',
          text: 'Invalid email or password.',
        });
      } finally {
        this.loading = false;
      }
    },

    forgotPassword() {
      alert('Forgot Password clicked');
    },

    goToRegisterPage() {
      this.$router.push('/register');
    },
  },
};
</script>

<style scoped>
.login-bg {
  background-image: url('@/assets/login-bg.jpg');
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  height: 100vh;
}

.overlay {
  background-color: rgba(255, 255, 255, 0.85);
  border-radius: 1rem;
  padding: 2rem;
  box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
}

@media (max-width: 768px) {
  .overlay {
    padding: 1.5rem;
  }
}
</style>
