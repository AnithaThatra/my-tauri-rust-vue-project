<template>
  <b-container fluid class="d-flex align-items-center justify-content-center bg-light p-0" style="height: 100vh;">
    <b-row
      class="shadow-lg rounded overflow-hidden w-100"
      style="max-width: 600px; min-height: 400px; height: auto; padding: 20px; margin-top: -50px;"
    >
      <!-- Edit User Form -->
      <b-col cols="12" class="bg-white p-4 d-flex align-items-center">
        <div class="w-100">
          <h4 class="fw-semibold text-center mb-4">Edit User</h4>

          <b-form @submit.prevent="updateUser">
            <b-form-group label="Name" label-for="name" class="mb-3">
              <b-form-input
                id="name"
                v-model="user.name"
                placeholder="Enter full name"
                class="rounded-pill py-2 px-3 shadow-sm"
                required
              />
            </b-form-group>

            <b-form-group label="Email" label-for="email" class="mb-3">
              <b-form-input
                id="email"
                type="email"
                v-model="user.email"
                placeholder="Enter email address"
                class="rounded-pill py-2 px-3 shadow-sm"
                required
              />
            </b-form-group>

            <b-form-group label="Role" label-for="role" class="mb-4">
              <b-form-select
                id="role"
                v-model="user.role"
                :options="roles"
                class="rounded-pill py-2 px-3 shadow-sm"
                required
              />
            </b-form-group>

            <!-- Buttons -->
            <b-row class="mt-4">
              <b-col class="d-flex justify-content-between">
                <b-button
                  variant="outline-primary"
                  @click="$router.push('/read')"
                  class="w-auto rounded-pill py-2 px-4"
                  :style="backButtonStyle"
                  @mouseover="hover = true"
                  @mouseleave="hover = false"
                >
                  Back to User List
                </b-button>

                <b-button
                  type="submit"
                  class="w-auto rounded-pill py-2 px-4 fw-bold text-white"
                  style="background: linear-gradient(to right, #6a11cb, #2575fc); border: none;"
                >
                  Update User
                </b-button>
              </b-col>
            </b-row>
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
  name: 'UpdateUser',
  data() {
    return {
      user: { id: null, name: '', email: '', role: '' },
      roles: ['admin', 'user', 'manager'],
      hover: false
    };
  },
  computed: {
    backButtonStyle() {
      return {
        backgroundColor: this.hover ? '#ff3d2e' : '#ff6f61',
        border: 'none',
        color: 'white',
        fontWeight: 'bold',
        transition: 'background-color 0.3s ease',
      };
    }
  },
  async created() {
    const token = localStorage.getItem('auth_token');
    if (!token) {
      Swal.fire('Access Denied', 'No auth token found.', 'error');
      this.$router.push('/login');
      return;
    }

    try {
      const all = await invoke('fetch_all_users_tauri', { token });
      const match = all.find(u => u.id === parseInt(this.$route.params.id));
      if (match) {
        this.user = match;
      } else {
        Swal.fire('Not Found', 'User not found.', 'warning');
        this.$router.push({ name: 'Read' });
      }
    } catch (err) {
      Swal.fire('Error', 'Failed to fetch users.', 'error');
      this.$router.push({ name: 'Read' });
    }
  },
  methods: {
    async updateUser() {
      const token = localStorage.getItem('auth_token');
      if (!token) {
        Swal.fire('Error', 'Authentication token missing.', 'error');
        return;
      }

      try {
        await invoke('update_user_tauri', {
          user: this.user,
          token
        });

        Swal.fire({
          icon: 'success',
          title: 'User updated!',
          showConfirmButton: false,
          timer: 1500
        });

       
      } catch (err) {
        Swal.fire({
          icon: 'error',
          title: 'Update Failed',
          text: typeof err === 'string' ? err : err.message || 'An error occurred.',
        });
      }
    }
  }
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
@media (max-width: 768px) {
  .shadow-lg {
    box-shadow: none;
  }
  .rounded {
    border-radius: 8px;
  }
}
</style>
