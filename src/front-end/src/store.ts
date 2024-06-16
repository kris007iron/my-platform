import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useStore = defineStore('store', () => {
    const projects = ref([
        {
            "_id": {
                "$oid": "6543ea5d875bc6bcda7d9218"
            },
            "title": "Rust Project with SurrealDB",
            "description": "Example of using SurrealDB with Rust as an API with three endpoints: returning all movies, one with specific id and posting a new review connected to specific movie.",
            "link": "https://github.com/kris007iron/movies-rust/",
            "images": [
                "@/assets/exampleIMG/csharp.png"
            ],
            "tags": [
                "rust",
                "surrealdb",
                "api",
                "backend"
            ]
        },
        {
            "_id": {
                "$oid": "654ab354bd65324f3b914e30"
            },
            "title": "Foodie - simple web dish macro counter - PL",
            "description": "Foodie allows users to create meal plans by adding available products, tracking calorie intake throughout the day, and monitoring macro-nutrient ratios without data preservation.",
            "link": "https://github.com/kris007iron/Foodie",
            "images": [
                "@/assets/exampleIMG/postgres.png"
            ],
            "tags": [
                ".NET",
                "JavaScript",
                "HTML",
                "CSS",
                "PostgreSQL"
            ]
        }
    ])

    const tags = ref([
        "rust",
        "surrealdb",
        "api",
        "backend",
        ".NET",
        "JavaScript",
        "HTML",
        "CSS",
        "PostgreSQL"
    ])

    const getProjects = computed(() => projects.value)

    const getTags = computed(() => tags.value)

    return {
        projects,
        getProjects,
        tags,
        getTags,
    }
})