<template>
  <b-container fluid class="bg-light py-5">
    <h2 class="text-center mb-4">User Management</h2>

    <!-- Search and Add Button -->
    <b-row class="align-items-center mb-3">
      <b-col cols="12" md="6">
        <b-input-group class="search-bar">
          <b-form-input
            v-model="searchQuery"
            placeholder="Search by name or email"
            class="rounded-pill shadow-sm"
          />
        </b-input-group>
      </b-col>
      <b-col
        cols="12"
        md="6"
        class="d-flex justify-content-md-end justify-content-center mt-2 mt-md-0"
      >
        <b-button
          variant="success"
          class="d-flex align-items-center gap-2 px-4 py-1 rounded-pill shadow-sm text-white"
          style="background-color: #28a745; border: none;"
          @click="$router.push('/create')"
        >
          <i class="bi bi-person-plus-fill fs-5"></i>
          <span>Add New</span>
        </b-button>
      </b-col>
    </b-row>

    <!-- User Table -->
    <b-table
      :items="paginatedUsers"
      :fields="['id', 'name', 'email', 'role', 'edit', 'delete']"
      hover
      responsive="sm"
      class="bg-white rounded shadow-sm"
    >
      <!-- Edit Column -->
      <template #cell(edit)="row">
        <b-button size="sm" variant="link" @click="editUser(row.item.id)">
          <i class="bi bi-pencil text-primary fs-5"></i>
        </b-button>
      </template>

      <!-- Delete Column -->
      <template #cell(delete)="row">
        <b-button size="sm" variant="link" @click="navigateToDelete(row.item.id)">
          <i class="bi bi-trash text-danger fs-5"></i>
        </b-button>
      </template>
    </b-table>

    <!-- Pagination -->
    <b-pagination
      v-model="currentPage"
      :total-rows="filteredUsers.length"
      :per-page="perPage"
      align="center"
      class="my-4"
      pills
      size="md"
    />

    <!-- Alert -->
    <b-alert
      :show="alert.show"
      :variant="alert.variant"
      dismissible
      @dismissed="alert.show = false"
    >
      {{ alert.message }}
    </b-alert>
  </b-container>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'UserManagement',
  data() {
    return {
      users: [],
      searchQuery: '',
      currentPage: 1,
      perPage: 5,
      alert: {
        show: false,
        variant: '',
        message: '',
      },
    };
  },
  computed: {
    filteredUsers() {
      return this.users.filter(
        (user) =>
          user.name.toLowerCase().includes(this.searchQuery.toLowerCase()) ||
          user.email.toLowerCase().includes(this.searchQuery.toLowerCase())
      );
    },
    paginatedUsers() {
      const start = (this.currentPage - 1) * this.perPage;
      return this.filteredUsers.slice(start, start + this.perPage);
    },
  },
  async created() {
    try {
      this.users = await invoke('fetch_all_users_tauri');
    } catch (error) {
      console.error('Failed to fetch users:', error);
      this.alert = {
        show: true,
        variant: 'danger',
        message: 'Failed to fetch users.',
      };
    }
  },
  methods: {
    editUser(id) {
      this.$router.push(`/update/${id}`);
    },
    navigateToDelete(id) {
      this.$router.push(`/delete/${id}`);
    },
  },
};
</script>

<style scoped>
.bg-light {
  background-color: #f8f9fa !important;
}

h2 {
  font-weight: bold;
}

.b-table th,
.b-table td {
  padding: 1rem;
  vertical-align: middle;
}

.b-table {
  background-color: white;
  border-radius: 10px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.05);
}

.b-form-input {
  font-size: 0.95rem;
}

.search-bar {
  max-width: 100%;
}

@media (min-width: 768px) {
  .search-bar {
    max-width: 300px;
  }
}
</style>
