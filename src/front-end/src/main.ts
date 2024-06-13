import { createApp } from 'vue'
import './assets/index.css'
import App from '@/App.vue'
import router from '@/router'
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faGithub } from '@fortawesome/free-brands-svg-icons'

library.add(faGithub)

createApp(App).component('font-awesome-icon', FontAwesomeIcon).use(router).mount('#app')
