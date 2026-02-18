import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import {
  faGear, faRotateRight, faPen, faCopy, faLink, faTrash,
  faFloppyDisk, faXmark, faPlus, faGlobe, faFolder,
  faTriangleExclamation, faLightbulb, faInbox,
  faBook, faTerminal, faBolt, faPlug, faCheck, faChevronDown, faChevronUp,
  faPuzzlePiece, faFileLines, faChartBar, faMagnifyingGlass, faSpinner
} from '@fortawesome/free-solid-svg-icons'

library.add(
  faGear, faRotateRight, faPen, faCopy, faLink, faTrash,
  faFloppyDisk, faXmark, faPlus, faGlobe, faFolder,
  faTriangleExclamation, faLightbulb, faInbox,
  faBook, faTerminal, faBolt, faPlug, faCheck, faChevronDown, faChevronUp,
  faPuzzlePiece, faFileLines, faChartBar, faMagnifyingGlass, faSpinner
)

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.component('font-awesome-icon', FontAwesomeIcon)
app.mount('#app')
