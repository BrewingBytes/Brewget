import { createRouter, createWebHistory } from "vue-router";

import { useAuthStore } from "@/stores/auth";
import AuthView from "@/views/AuthView.vue";
import HomeView from "@/views/HomeView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/login",
      name: "login",
      component: AuthView,
    },
  ],
});

router.beforeEach((to) => {
  if (!useAuthStore().isAuthenticated && to.name !== "login") {
    return { name: "login" };
  }
});

export default router;
