import { createApp } from 'vue'
import './assets/index.css'
import App from '@/App.vue'
import router from '@/router'
import { createPinia } from 'pinia'
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faGithub, faRust, faMicrosoft, faJs, faHtml5, faCss3Alt } from '@fortawesome/free-brands-svg-icons'
import { faDatabase, faServer, faPlug } from '@fortawesome/free-solid-svg-icons'


const pinia = createPinia()
library.add(faGithub)
library.add(faRust)
library.add(faMicrosoft)
library.add(faJs)
library.add(faHtml5)
library.add(faCss3Alt)
library.add(faDatabase)
library.add(faServer)
library.add(faPlug)

createApp(App).component('font-awesome-icon', FontAwesomeIcon).use(router).use(pinia).mount('#app')
