import { type RouteRecordNameGeneric, createRouter, createWebHistory } from "vue-router";

import { useAuthStore } from "@/stores/auth";
import AuthView from "@/views/AuthView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/login",
      name: "login",
      component: AuthView,
    },
    {
      path: "/forgot-password/:id",
      name: "forgot-password",
      component: AuthView,
    },
  ],
});

router.beforeEach((to) => {
  if (!useAuthStore().isAuthenticated && isAuthRoute(to.name)) {
    return { name: "login" };
  }
});

function isAuthRoute(name: RouteRecordNameGeneric) {
  switch (name) {
    case "login":
    case "forgot-password":
      return false;
    default:
      return true;
  }
}

export default router;
