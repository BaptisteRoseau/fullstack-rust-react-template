import { AppProvider } from './AppProvider';
import { AppRouter } from './AppRouter';
import './utils.css'

function App() {
    return (
        <AppProvider>
            <AppRouter />
        </AppProvider>
    )
}

export default App
