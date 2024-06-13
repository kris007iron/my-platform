<template>
  <div class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-br to-background from-foreground text-center text-foreground">
    <div class="p-8">
      <h1 class="text-9xl font-extrabold text-primary mb-4">404</h1>
      <p class="text-2xl mb-8">Oops! The page you are looking for does not exist.</p>
      <img src="@/assets/404-illustration.svg" alt="Not Found" class="w-1/2 mx-auto mb-8" />
      <div class="space-x-4">
        <router-link to="/" class="bg-primary text-primary-foreground py-2 px-4 rounded-lg hover:bg-primary-foreground hover:text-primary">
          Go Home
        </router-link>
        <router-link to="/about" class="text-muted-foreground hover:text-primary">
          Learn More About Us
        </router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped>
/* In your custom.css or scoped in NotFound.vue */

@keyframes fadeIn {
  0% { opacity: 0; }
  100% { opacity: 1; }
}

@keyframes bounce {
  0%, 100% { transform: translateY(-25%); }
  50% { transform: translateY(0); }
}

body {
  cursor: url('/path/to/custom-cursor.png'), auto; /* Example of custom cursor */
}

.text-9xl {
  animation: bounce 1s infinite;
}

.text-primary {
  animation: fadeIn 1s ease-in;
}

.router-link {
  transition: background-color 0.3s, color 0.3s;
}

.router-link:hover {
  background-color: var(--primary-foreground);
  color: var(--primary);
}

</style>