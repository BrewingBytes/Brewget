import { type RouteRecordNameGeneric, createRouter, createWebHistory } from "vue-router";

import { useAuthStore } from "@/stores/auth";

// Lazy load views for better initial page load performance
const AuthView = () => import("@/views/AuthView.vue");
const HomeView = () => import("@/views/HomeView.vue");
const SettingsView = () => import("@/views/SettingsView.vue");
const WalletsView = () => import("@/views/WalletsView.vue");

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/wallets",
      name: "wallets",
      component: WalletsView,
    },
    {
      path: "/add",
      name: "add",
      component: HomeView,
    },
    {
      path: "/analytics",
      name: "analytics",
      component: HomeView,
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingsView,
    },
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
    {
      path: "/activate/:id",
      name: "activate",
      component: AuthView,
    },
  ],
});

router.beforeEach((to) => {
  if (!useAuthStore().isAuthenticated && isAuthRoute(to.name)) {
    return { name: "login" };
  }
});

export function isAuthRoute(name: RouteRecordNameGeneric) {
  switch (name) {
    case "activate":
    case "forgot-password":
    case "login":
      return false;
    default:
      return true;
  };
};

export default router;
