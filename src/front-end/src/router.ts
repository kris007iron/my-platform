import { createRouter, createWebHistory, Router } from "vue-router";
import HomeView from "@/pages/HomeView.vue";

const routes = [
    { path: "/home", component: HomeView },
]

const router: Router = createRouter({ history: createWebHistory(import.meta.env.BASE_URL), routes })

export default router