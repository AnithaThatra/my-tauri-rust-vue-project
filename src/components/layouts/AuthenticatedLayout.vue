<template>
  <b-container fluid class="p-0">
    <!-- Topbar -->
    <b-navbar toggleable="lg" type="light" variant="light" class="shadow-sm px-3 bg-white">
      <b-navbar-brand class="fw-bold text-primary">User Dashboard</b-navbar-brand>
      <b-navbar-brand class="d-flex align-items-center gap-2">
        <img src="/icons/tauri.svg" alt="Tauri" height="24" />
        <img src="/icons/rust.svg" alt="Rust" height="24" />
        <img src="/icons/vue.svg" alt="Vue" height="24" />
        <img src="/icons/vite.svg" alt="Vite" height="24" />
      </b-navbar-brand>
      <b-navbar-toggle target="nav-collapse" />
      <b-collapse id="nav-collapse" is-nav class="ms-auto">
        <b-nav class="ms-auto d-flex align-items-center">
          <b-nav-item>
            <i class="bi bi-bell fs-5 text-primary"></i>
          </b-nav-item>
          <b-nav-item class="ms-3">
            <b-dropdown right no-caret variant="link" toggle-class="p-0" menu-class="shadow-sm">
              <template #button-content>
                <b-avatar
                  src="https://randomuser.me/api/portraits/women/44.jpg"
                  size="32px"
                  rounded
                ></b-avatar>
              </template>
              <div class="px-3 py-2 border-bottom text-center">
                <strong>{{ user.name }}</strong><br />
                <small class="text-muted">{{ user.role }}</small>
              </div>
              <b-dropdown-item @click="logout">
                <i class="bi bi-box-arrow-right me-2 text-danger" /> Logout
              </b-dropdown-item>
            </b-dropdown>
          </b-nav-item>
        </b-nav>
      </b-collapse>
    </b-navbar>

    <!-- Layout -->
    <b-row no-gutters>
      <!-- Sidebar -->
      <b-col cols="12" md="3" lg="2" class="bg-light border-end vh-100 d-flex flex-column pt-4">
        <div class="text-center mb-4 fw-bold h5 text-primary">
          <i class="bi bi-people-fill fs-4 mb-1" />
          <div>Users</div>
        </div>
        <b-nav vertical pills class="flex-column">
          <b-nav-item to="/dashboard" active-class="active" class="text-dark px-3 py-2">
              <i class="bi bi-speedometer2 me-2 text-secondary"></i>
                Dashboard
            </b-nav-item>
           <b-nav-item to="/read" active-class="active" class="text-dark px-3 py-2">
            <i class="bi bi-eye me-2 text-secondary" /> View Users
          </b-nav-item>
          <b-nav-item @click="logout" class="text-dark px-3 py-2" style="cursor: pointer;">
            <i class="bi bi-box-arrow-right me-2 text-secondary" /> Logout
          </b-nav-item>
        </b-nav>
      </b-col>

      <!-- Main content -->
      <b-col cols="12" md="9" lg="10" class="p-4 bg-white">
        <router-view />
      </b-col>
    </b-row>
  </b-container>
</template>

<script>
import Swal from 'sweetalert2';

export default {
  name: 'AuthenticatedLayout',
  data() {
    return {
      user: {
        name: 'Jane Doe',
        role: 'Admin',
      },
    };
  },
  created() {
    const storedUser = localStorage.getItem('user_info');
    if (storedUser) {
      this.user = JSON.parse(storedUser);
    }
  },
  methods: {
    async logout() {
      const result = await Swal.fire({
        title: 'Are you sure?',
        text: 'Do you really want to logout?',
        icon: 'warning',
        showCancelButton: true,
        confirmButtonText: 'Yes, logout',
        cancelButtonText: 'Cancel',
        confirmButtonColor: '#dc3545',
      });

      if (result.isConfirmed) {
        localStorage.removeItem('auth_token');
        localStorage.removeItem('user_info');
        this.$router.push({ name: 'Login' });
      }
    },
  },
};
</script>

<style scoped>
.bg-light {
  background-color: #f8f9fa !important;
}
.vh-100 {
  height: 100vh !important;
}
.text-primary {
  color: #007bff !important;
}
.text-secondary {
  color: #6c757d !important;
}
.cursor-pointer {
  cursor: pointer;
}
.fw-bold {
  font-weight: bold;
}
.h5 {
  font-size: 1.25rem;
}
.mb-1 {
  margin-bottom: 0.25rem !important;
}
.text-danger {
  color: #dc3545 !important;
}
.navbar-nav {
  display: flex;
  align-items: center;
}
.navbar-nav .nav-item {
  padding: 0.5rem 1rem;
}
.navbar-nav .nav-item i {
  margin-right: 8px;
}

.text-center {
  text-align: center;
}

.nav-item .bi {
  vertical-align: middle;
}
</style>
