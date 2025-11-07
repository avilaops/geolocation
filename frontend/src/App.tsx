import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import { Toaster } from 'react-hot-toast'
import Layout from './components/Layout'
import Dashboard from './pages/Dashboard'
import Upload from './pages/Upload'
import NotasFiscais from './pages/NotasFiscais'
import ConhecimentosTransporte from './pages/ConhecimentosTransporte'
import AnalyseFiscal from './pages/AnalyseFiscal'
import Settings from './pages/Settings'

function App() {
    return (
        <Router>
            <Toaster
                position="top-right"
                toastOptions={{
                    duration: 4000,
                    style: {
                        background: '#363636',
                        color: '#fff',
                    },
                    success: {
                        duration: 3000,
                        iconTheme: {
                            primary: '#10b981',
                            secondary: '#fff',
                        },
                    },
                    error: {
                        duration: 4000,
                        iconTheme: {
                            primary: '#ef4444',
                            secondary: '#fff',
                        },
                    },
                }}
            />
            <Layout>
                <Routes>
                    <Route path="/" element={<Dashboard />} />
                    <Route path="/upload" element={<Upload />} />
                    <Route path="/notas-fiscais" element={<NotasFiscais />} />
                    <Route path="/ctes" element={<ConhecimentosTransporte />} />
                    <Route path="/analise-fiscal" element={<AnalyseFiscal />} />
                    <Route path="/settings" element={<Settings />} />
                </Routes>
            </Layout>
        </Router>
    )
}

export default App
