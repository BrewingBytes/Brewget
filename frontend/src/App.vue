<template>
  <div class="container">
    <h1>Vue 3 + PrimeVue Frontend</h1>
    <div class="health-check">
      <Button label="Check Backend Health" @click="checkHealth" />
      <div v-if="healthStatus" class="status-message">
        {{ healthStatus }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import Button from 'primevue/button'
import axios from 'axios'

const healthStatus = ref('')

const checkHealth = async () => {
  try {
    const response = await axios.get('/api/health')
    healthStatus.value = `Backend Status: ${response.data.status} - ${response.data.message}`
  } catch (error) {
    healthStatus.value = 'Error connecting to backend'
  }
}
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}

.health-check {
  margin-top: 2rem;
}

.status-message {
  margin-top: 1rem;
  font-size: 1.1rem;
  color: #2196f3;
}
</style>