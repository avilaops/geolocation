import { useEffect } from 'react'
import {
    FileText,
    Truck,
    TrendingUp,
    Activity,
    Sparkles,
} from 'lucide-react'
import {
    LineChart,
    Line,
    BarChart,
    Bar,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
    ResponsiveContainer,
} from 'recharts'
import StatCard from '../components/StatCard'
import DocumentTable from '../components/DocumentTable'
import { useStore } from '../store/useStore'
import { documentService, DocumentSummary } from '../services/api'
import toast from 'react-hot-toast'

// Dados simulados para os gráficos
const chartData = [
    { mes: 'Jan', nfe: 120, cte: 80 },
    { mes: 'Fev', nfe: 150, cte: 95 },
    { mes: 'Mar', nfe: 180, cte: 110 },
    { mes: 'Abr', nfe: 160, cte: 100 },
    { mes: 'Mai', nfe: 200, cte: 130 },
    { mes: 'Jun', nfe: 220, cte: 145 },
]

export default function Dashboard() {
    const { stats, documents, setStats, setDocuments, setLoading } = useStore()

    useEffect(() => {
        loadDashboardData()
    }, [])

    const loadDashboardData = async () => {
        setLoading(true)
        try {
            const [statsData, docsData] = await Promise.all([
                documentService.getStats(),
                documentService.listDocuments({ limit: 50 }),
            ])

            setStats({
                totalDocuments: statsData.total_documents,
                processedToday: statsData.processed_today,
                notasFiscais: statsData.notas_fiscais,
                ctes: statsData.ctes,
            })

            // Adapta DocumentSummary para o formato interno esperado pelo store
            const mapped = (docsData as DocumentSummary[]).map((d) => ({
                id: d.chave_acesso, // usa chave como identificador
                tipo: (d.document_type === 'NFe' ? 'NFe' : 'CTe') as 'NFe' | 'CTe',
                chave_acesso: d.chave_acesso,
                numero: d.numero,
                serie: d.serie,
                data_emissao: d.data_emissao,
                emitente: d.emitente,
                destinatario: d.destinatario,
                valor_total: d.valor_total,
                status: 'Completed' as const,
            }))
            setDocuments(mapped)
            toast.success('Dashboard atualizado!')
        } catch (error) {
            console.error('Erro ao carregar dashboard:', error)
            toast.error('Erro ao carregar dados do dashboard')

            // Dados de exemplo para desenvolvimento
            setStats({
                totalDocuments: 1247,
                processedToday: 42,
                notasFiscais: 850,
                ctes: 397,
            })
        } finally {
            setLoading(false)
        }
    }

    return (
        <div className="space-y-8 animate-fade-in">
            {/* Título com gradiente */}
            <div className="relative">
                <div className="absolute inset-0 bg-gradient-to-r from-indigo-600 to-purple-600 opacity-10 blur-3xl" />
                <div className="relative">
                    <h1 className="text-4xl font-extrabold bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent">
                        Dashboard
                    </h1>
                    <p className="text-gray-500 mt-2 text-lg font-medium flex items-center gap-2">
                        <Sparkles className="w-5 h-5 text-indigo-500" />
                        Visão geral dos documentos fiscais em tempo real
                    </p>
                </div>
            </div>

            {/* Cards de Estatísticas com delay escalonado */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <StatCard
                    title="Total de Documentos"
                    value={stats.totalDocuments}
                    icon={Activity}
                    color="blue"
                    trend={{ value: 12, isPositive: true }}
                    delay={0}
                />
                <StatCard
                    title="Processados Hoje"
                    value={stats.processedToday}
                    icon={TrendingUp}
                    color="green"
                    trend={{ value: 8, isPositive: true }}
                    delay={100}
                />
                <StatCard
                    title="Notas Fiscais (NF-e)"
                    value={stats.notasFiscais}
                    icon={FileText}
                    color="purple"
                    trend={{ value: 5, isPositive: true }}
                    delay={200}
                />
                <StatCard
                    title="Conhecimentos (CT-e)"
                    value={stats.ctes}
                    icon={Truck}
                    color="orange"
                    trend={{ value: 3, isPositive: false }}
                    delay={300}
                />
            </div>

            {/* Gráficos */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {/* Gráfico de Linha */}
                <div className="card">
                    <h3 className="text-lg font-semibold text-gray-900 mb-4">
                        Tendência de Documentos
                    </h3>
                    <ResponsiveContainer width="100%" height={300}>
                        <LineChart data={chartData}>
                            <CartesianGrid strokeDasharray="3 3" />
                            <XAxis dataKey="mes" />
                            <YAxis />
                            <Tooltip />
                            <Legend />
                            <Line
                                type="monotone"
                                dataKey="nfe"
                                stroke="#8b5cf6"
                                strokeWidth={2}
                                name="NF-e"
                            />
                            <Line
                                type="monotone"
                                dataKey="cte"
                                stroke="#f97316"
                                strokeWidth={2}
                                name="CT-e"
                            />
                        </LineChart>
                    </ResponsiveContainer>
                </div>

                {/* Gráfico de Barras */}
                <div className="card">
                    <h3 className="text-lg font-semibold text-gray-900 mb-4">
                        Comparativo Mensal
                    </h3>
                    <ResponsiveContainer width="100%" height={300}>
                        <BarChart data={chartData}>
                            <CartesianGrid strokeDasharray="3 3" />
                            <XAxis dataKey="mes" />
                            <YAxis />
                            <Tooltip />
                            <Legend />
                            <Bar dataKey="nfe" fill="#8b5cf6" name="NF-e" />
                            <Bar dataKey="cte" fill="#f97316" name="CT-e" />
                        </BarChart>
                    </ResponsiveContainer>
                </div>
            </div>

            <DocumentTable pageSize={20} />
        </div>
    )
}
