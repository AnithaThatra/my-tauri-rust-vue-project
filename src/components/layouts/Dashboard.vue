<template>
  <b-container fluid class="py-4">
    <!-- Header -->
    <h2 class="mb-4 text-primary fw-bold text-center text-md-start">Admin Dashboard</h2>

    <!-- Summary Cards -->
    <b-row class="g-3">
      <b-col cols="12" md="4">
        <b-card
          bg-variant="primary"
          text-variant="white"
          class="text-center shadow-sm rounded-3"
        >
          <h5>Total Users</h5>
          <h2>{{ stats.totalUsers }}</h2>
        </b-card>
      </b-col>
      <b-col cols="12" md="4">
        <b-card
          bg-variant="success"
          text-variant="white"
          class="text-center shadow-sm rounded-3"
        >
          <h5>Active Admins</h5>
          <h2>{{ stats.admins }}</h2>
        </b-card>
      </b-col>
      <b-col cols="12" md="4">
        <b-card
          bg-variant="warning"
          text-variant="white"
          class="text-center shadow-sm rounded-3"
        >
          <h5>Pending Tasks</h5>
          <h2>{{ stats.tasks }}</h2>
        </b-card>
      </b-col>
    </b-row>

    <!-- Charts Section -->
    <b-row class="mt-4">
      <!-- Bar Chart (Monthly Registrations) -->
      <b-col cols="12" md="6">
        <b-card class="shadow-sm rounded-3">
          <h5 class="mb-3">Monthly Registrations</h5>
          <Bar v-if="loaded" :data="barChartData" :options="chartOptions" />
        </b-card>
      </b-col>

      <!-- Doughnut Chart (User Role Distribution) -->
      <b-col cols="12" md="6" class="d-flex justify-content-center align-items-center">
        <b-card class="shadow-sm rounded-3" style="min-height: 300px;">
          <h5 class="mb-3">User Roles</h5>
          <Doughnut v-if="loaded" :data="doughnutChartData" :options="doughnutChartOptions" />
        </b-card>
      </b-col>
    </b-row>

    <!-- Recent Activity Section -->
    <b-row class="mt-4">
      <b-col cols="12">
        <b-card class="shadow-sm rounded-3">
          <h5 class="mb-3">Recent Activity</h5>
          <p class="text-muted">Recent logs, updates, or a chart will be displayed here.</p>
        </b-card>
      </b-col>
    </b-row>
  </b-container>
</template>

<script>
import { Bar, Doughnut } from 'vue-chartjs';
import {
  Chart as ChartJS, Title, Tooltip, Legend,
  BarElement, ArcElement, CategoryScale, LinearScale
} from 'chart.js';
import { invoke } from '@tauri-apps/api/core';

ChartJS.register(Title, Tooltip, Legend, BarElement, ArcElement, CategoryScale, LinearScale);

export default {
  name: 'Dashboard',
  components: { Bar, Doughnut },
  data() {
    return {
      stats: {
        totalUsers: 0,
        admins: 0,
        tasks: 8, // Static value for now, you can update with actual data
      },
      barChartData: null,
      doughnutChartData: null,
      chartOptions: {
        responsive: true,
        plugins: {
          legend: { position: 'top' },
          title: { display: true, text: 'User Data Overview' },
        }
      },
      doughnutChartOptions: {
        responsive: true,
        plugins: {
          legend: { position: 'top' },
          title: { display: true, text: 'Role Distribution' },
        }
      },
      loaded: false,
    };
  },
  async mounted() {
    const token = localStorage.getItem('auth_token');
    
    // Fetch user data
    const users = await invoke('fetch_all_users_tauri', { token });
    this.stats.totalUsers = users.length;
    this.stats.admins = users.filter(user => user.role?.toLowerCase() === 'admin').length;

    // Prepare Monthly Registration Data for Bar Chart
    const monthlyCounts = Array(12).fill(0);
    users.forEach(user => {
      const date = new Date(user.created_at || user.createdAt);
      if (!isNaN(date)) monthlyCounts[date.getMonth()] += 1;
    });
    this.barChartData = {
      labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
      datasets: [{
        label: 'Registrations',
        data: monthlyCounts,
        backgroundColor: '#6a11cb'
      }]
    };

    // Prepare User Roles Data for Doughnut Chart
    const roleCount = {};
    users.forEach(user => {
      const role = (user.role || 'unknown').toLowerCase();
      roleCount[role] = (roleCount[role] || 0) + 1;
    });
    this.doughnutChartData = {
      labels: Object.keys(roleCount),
      datasets: [{
        data: Object.values(roleCount),
        backgroundColor: ['#6a11cb', '#2575fc', '#ff6f61', '#ffc107', '#00c851']
      }]
    };

    this.loaded = true;
  },
};
</script>

<style scoped>
h2, h5 {
  font-weight: 600;
}

.card-body {
  padding: 1.5rem;
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
