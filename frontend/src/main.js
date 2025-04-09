import { createApp } from 'vue'
import App from './App.vue'
import PrimeVue from 'primevue/config'
import Lara from '@primeuix/themes/lara'

const app = createApp(App)
app.use(PrimeVue, {
    theme: {
        preset: Lara
    }
})
app.mount('#app')
