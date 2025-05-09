<template>
  <b-container class="mt-4">
    <h2>Confirm Delete</h2>
    <p>
      Are you sure you want to delete the user with ID <strong>{{ userId }}</strong>?
    </p>

    <b-button variant="danger" @click="showModal = true">Yes, Delete</b-button>
    <b-button variant="secondary" @click="this.$router.push({ name: 'Read' })">Cancel</b-button>

    <b-modal
      v-model="showModal"
      title="Confirm Deletion"
      ok-title="Delete"
      ok-variant="danger"
      cancel-title="Cancel"
      @ok="deleteUser"
    >
      <p>This action is irreversible. Do you really want to proceed?</p>
    </b-modal>

    <b-alert
      :show="alert.show"
      :variant="alert.variant"
      dismissible
      @dismissed="alert.show = false"
      class="mt-3"
    >
      {{ alert.message }}
    </b-alert>
  </b-container>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'DeleteUser',
  data() {
    return {
      userId: parseInt(this.$route.params.id),
      showModal: false,
      alert: {
        show: false,
        variant: '',
        message: '',
      },
    };
  },
  methods: {
    async deleteUser() {
      const token = localStorage.getItem('auth_token');
      if (!token) {
        this.alert = {
          show: true,
          variant: 'danger',
          message: 'Admin authentication token is missing.',
        };
        return;
      }

      try {
        await invoke('delete_user_tauri', {
          id: this.userId,
          token: token,
        });

        this.alert = {
          show: true,
          variant: 'success',
          message: 'User deleted successfully.',
        };

        // Redirect after short delay to show success
        setTimeout(() => this.$router.push('/read'), 1000);
      } catch (err) {
        this.alert = {
          show: true,
          variant: 'danger',
          message: 'Failed to delete user: ' + err,
        };
      }
    },
  },
};
</script>
