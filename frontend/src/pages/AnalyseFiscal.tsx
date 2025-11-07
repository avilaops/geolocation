import { useEffect, useState } from 'react'
import {
    DollarSign,
    TrendingUp,
    TrendingDown,
    AlertTriangle,
    FileText,
    PieChart as PieChartIcon,
    BarChart3,
    Calendar,
    Download,
    RefreshCw,
} from 'lucide-react'
import {
    PieChart,
    Pie,
    Cell,
    BarChart,
    Bar,
    LineChart,
    Line,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    Legend,
    ResponsiveContainer,
} from 'recharts'
import toast from 'react-hot-toast'

interface TaxAnalysis {
    periodo: string
    icms: number
    pis: number
    cofins: number
    total: number
    documentos: number
}

interface CFOPDistribution {
    cfop: string
    descricao: string
    quantidade: number
    valor: number
    percentual: number
}

interface Alert {
    tipo: 'error' | 'warning' | 'info'
    titulo: string
    descricao: string
    documento?: string
}

export default function AnalyseFiscal() {
    const [loading, setLoading] = useState(false)
    const [periodo, setPeriodo] = useState('2024-11')
    const [taxAnalysis, setTaxAnalysis] = useState<TaxAnalysis[]>([])
    const [cfopDistribution, setCfopDistribution] = useState<CFOPDistribution[]>([])
    const [alerts, setAlerts] = useState<Alert[]>([])

    useEffect(() => {
        loadFiscalData()
    }, [periodo])

    const loadFiscalData = async () => {
        setLoading(true)
        try {
            // Simulação de dados - em produção viria do backend
            await new Promise(resolve => setTimeout(resolve, 1000))

            // Dados de análise tributária dos últimos 6 meses
            setTaxAnalysis([
                { periodo: '2024-06', icms: 45000, pis: 8500, cofins: 39100, total: 92600, documentos: 156 },
                { periodo: '2024-07', icms: 52000, pis: 9800, cofins: 45100, total: 106900, documentos: 178 },
                { periodo: '2024-08', icms: 48000, pis: 9100, cofins: 41800, total: 98900, documentos: 165 },
                { periodo: '2024-09', icms: 58000, pis: 11000, cofins: 50600, total: 119600, documentos: 195 },
                { periodo: '2024-10', icms: 61000, pis: 11500, cofins: 53000, total: 125500, documentos: 210 },
                { periodo: '2024-11', icms: 55000, pis: 10400, cofins: 47900, total: 113300, documentos: 189 },
            ])

            // Distribuição por CFOP
            setCfopDistribution([
                { cfop: '5102', descricao: 'Venda de mercadoria', quantidade: 85, valor: 450000, percentual: 45 },
                { cfop: '6102', descricao: 'Venda interestadual', quantidade: 62, valor: 320000, percentual: 32 },
                { cfop: '5405', descricao: 'Venda de bem do ativo', quantidade: 18, valor: 120000, percentual: 12 },
                { cfop: '6108', descricao: 'Venda não contribuinte', quantidade: 24, valor: 110000, percentual: 11 },
            ])

            // Alertas fiscais
            setAlerts([
                {
                    tipo: 'error',
                    titulo: 'CFOP Inválido Detectado',
                    descricao: '3 documentos com CFOP 5102 para operação interestadual',
                    documento: 'NFe 35240811223344000156550010000123451234567890',
                },
                {
                    tipo: 'warning',
                    titulo: 'Divergência de Alíquota',
                    descricao: 'ICMS com alíquota 12% quando deveria ser 18% (operação interna SP)',
                    documento: 'NFe 35240911223344000156550010000123461234567891',
                },
                {
                    tipo: 'warning',
                    titulo: 'NCM sem Configuração',
                    descricao: '5 documentos com NCM 84331900 sem alíquota de IPI configurada',
                },
                {
                    tipo: 'info',
                    titulo: 'Oportunidade de Crédito',
                    descricao: 'R$ 12.450,00 em créditos de ICMS disponíveis não aproveitados',
                },
            ])

            toast.success('✨ Análise fiscal atualizada com sucesso!', {
                duration: 3000,
                icon: '📊',
                style: {
                    background: 'linear-gradient(135deg, #10b981 0%, #059669 100%)',
                    color: '#fff',
                    fontWeight: '600',
                    fontSize: '14px',
                    borderRadius: '16px',
                    boxShadow: '0 10px 30px rgba(16, 185, 129, 0.3)',
                    border: '2px solid #34d399',
                },
            })
        } catch (error) {
            toast.error('Erro ao carregar análise fiscal', {
                duration: 4000,
                style: {
                    background: 'linear-gradient(135deg, #ef4444 0%, #dc2626 100%)',
                    color: '#fff',
                    fontWeight: '600',
                    fontSize: '14px',
                    borderRadius: '16px',
                    boxShadow: '0 10px 30px rgba(239, 68, 68, 0.3)',
                    border: '2px solid #f87171',
                },
            })
        } finally {
            setLoading(false)
        }
    }

    const exportReport = () => {
        toast.success('📥 Exportando relatório em Excel...', {
            duration: 3000,
            icon: '📊',
            style: {
                background: 'linear-gradient(135deg, #3b82f6 0%, #2563eb 100%)',
                color: '#fff',
                fontWeight: '600',
                fontSize: '14px',
                borderRadius: '16px',
                boxShadow: '0 10px 30px rgba(59, 130, 246, 0.3)',
                border: '2px solid #60a5fa',
            },
        })
    }

    const COLORS = ['#6366f1', '#8b5cf6', '#ec4899', '#f59e0b']

    const currentMonth = taxAnalysis[taxAnalysis.length - 1]
    const previousMonth = taxAnalysis[taxAnalysis.length - 2]
    const variation = currentMonth && previousMonth
        ? ((currentMonth.total - previousMonth.total) / previousMonth.total) * 100
        : 0

    return (
        <div className="space-y-6 animate-fade-in">
            {/* Header Premium */}
            <div className="relative bg-gradient-to-r from-indigo-600 via-purple-600 to-pink-600 rounded-3xl shadow-2xl overflow-hidden">
                <div className="absolute inset-0 bg-black opacity-10" />
                <div className="relative px-8 py-8">
                    <div className="flex items-center justify-between">
                        <div>
                            <h1 className="text-4xl font-bold text-white mb-2 flex items-center gap-3">
                                <BarChart3 className="w-10 h-10" />
                                Análise Fiscal Inteligente
                            </h1>
                            <p className="text-white/90 text-lg">
                                Insights tributários e oportunidades de economia
                            </p>
                        </div>
                        <div className="flex gap-3">
                            <button
                                onClick={loadFiscalData}
                                disabled={loading}
                                className="px-5 py-3 bg-white/20 hover:bg-white/30 backdrop-blur-md text-white rounded-xl font-semibold 
                                         transition-all duration-300 hover:scale-105 disabled:opacity-50 flex items-center gap-2"
                            >
                                <RefreshCw className={`w-5 h-5 ${loading ? 'animate-spin' : ''}`} />
                                Atualizar
                            </button>
                            <button
                                onClick={exportReport}
                                className="px-5 py-3 bg-white text-indigo-600 hover:text-indigo-700 rounded-xl font-semibold 
                                         transition-all duration-300 hover:scale-105 hover:shadow-xl flex items-center gap-2"
                            >
                                <Download className="w-5 h-5" />
                                Exportar
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            {/* Cards de Resumo */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
                {/* Total de Impostos */}
                <div className="group relative bg-white rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-300 hover:-translate-y-2 p-6 animate-slide-up">
                    <div className="absolute inset-0 bg-gradient-to-br from-red-500/10 to-orange-500/10 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity" />
                    <div className="relative">
                        <div className="flex items-center justify-between mb-4">
                            <div className="p-3 bg-gradient-to-br from-red-500 to-orange-600 rounded-xl shadow-lg">
                                <DollarSign className="w-6 h-6 text-white" />
                            </div>
                            {variation !== 0 && (
                                <div className={`flex items-center gap-1 px-2 py-1 rounded-full text-xs font-bold
                                    ${variation > 0 ? 'bg-red-100 text-red-700' : 'bg-green-100 text-green-700'}`}>
                                    {variation > 0 ? <TrendingUp className="w-3 h-3" /> : <TrendingDown className="w-3 h-3" />}
                                    {Math.abs(variation).toFixed(1)}%
                                </div>
                            )}
                        </div>
                        <h3 className="text-gray-600 text-sm font-medium mb-1">Total de Impostos</h3>
                        <p className="text-3xl font-bold text-gray-900 group-hover:scale-105 transition-transform">
                            {currentMonth ? `R$ ${(currentMonth.total / 1000).toFixed(1)}k` : 'R$ 0'}
                        </p>
                        <p className="text-xs text-gray-500 mt-2">
                            {currentMonth?.documentos || 0} documentos fiscais
                        </p>
                    </div>
                </div>

                {/* ICMS */}
                <div className="group relative bg-white rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-300 hover:-translate-y-2 p-6 animate-slide-up" style={{ animationDelay: '100ms' }}>
                    <div className="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-indigo-500/10 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity" />
                    <div className="relative">
                        <div className="flex items-center justify-between mb-4">
                            <div className="p-3 bg-gradient-to-br from-blue-500 to-indigo-600 rounded-xl shadow-lg">
                                <FileText className="w-6 h-6 text-white" />
                            </div>
                        </div>
                        <h3 className="text-gray-600 text-sm font-medium mb-1">ICMS</h3>
                        <p className="text-3xl font-bold text-gray-900 group-hover:scale-105 transition-transform">
                            {currentMonth ? `R$ ${(currentMonth.icms / 1000).toFixed(1)}k` : 'R$ 0'}
                        </p>
                        <p className="text-xs text-gray-500 mt-2">
                            {currentMonth ? ((currentMonth.icms / currentMonth.total) * 100).toFixed(0) : 0}% do total
                        </p>
                    </div>
                </div>

                {/* PIS */}
                <div className="group relative bg-white rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-300 hover:-translate-y-2 p-6 animate-slide-up" style={{ animationDelay: '200ms' }}>
                    <div className="absolute inset-0 bg-gradient-to-br from-green-500/10 to-emerald-500/10 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity" />
                    <div className="relative">
                        <div className="flex items-center justify-between mb-4">
                            <div className="p-3 bg-gradient-to-br from-green-500 to-emerald-600 rounded-xl shadow-lg">
                                <PieChartIcon className="w-6 h-6 text-white" />
                            </div>
                        </div>
                        <h3 className="text-gray-600 text-sm font-medium mb-1">PIS</h3>
                        <p className="text-3xl font-bold text-gray-900 group-hover:scale-105 transition-transform">
                            {currentMonth ? `R$ ${(currentMonth.pis / 1000).toFixed(1)}k` : 'R$ 0'}
                        </p>
                        <p className="text-xs text-gray-500 mt-2">
                            {currentMonth ? ((currentMonth.pis / currentMonth.total) * 100).toFixed(0) : 0}% do total
                        </p>
                    </div>
                </div>

                {/* COFINS */}
                <div className="group relative bg-white rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-300 hover:-translate-y-2 p-6 animate-slide-up" style={{ animationDelay: '300ms' }}>
                    <div className="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-pink-500/10 rounded-2xl opacity-0 group-hover:opacity-100 transition-opacity" />
                    <div className="relative">
                        <div className="flex items-center justify-between mb-4">
                            <div className="p-3 bg-gradient-to-br from-purple-500 to-pink-600 rounded-xl shadow-lg">
                                <BarChart3 className="w-6 h-6 text-white" />
                            </div>
                        </div>
                        <h3 className="text-gray-600 text-sm font-medium mb-1">COFINS</h3>
                        <p className="text-3xl font-bold text-gray-900 group-hover:scale-105 transition-transform">
                            {currentMonth ? `R$ ${(currentMonth.cofins / 1000).toFixed(1)}k` : 'R$ 0'}
                        </p>
                        <p className="text-xs text-gray-500 mt-2">
                            {currentMonth ? ((currentMonth.cofins / currentMonth.total) * 100).toFixed(0) : 0}% do total
                        </p>
                    </div>
                </div>
            </div>

            {/* Gráficos */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {/* Evolução Tributária */}
                <div className="bg-white rounded-2xl shadow-lg p-6 animate-slide-up" style={{ animationDelay: '400ms' }}>
                    <h3 className="text-xl font-bold text-gray-900 mb-6 flex items-center gap-2">
                        <TrendingUp className="w-6 h-6 text-indigo-600" />
                        Evolução Tributária (6 meses)
                    </h3>
                    <ResponsiveContainer width="100%" height={300}>
                        <LineChart data={taxAnalysis}>
                            <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                            <XAxis dataKey="periodo" stroke="#6b7280" />
                            <YAxis stroke="#6b7280" />
                            <Tooltip
                                contentStyle={{
                                    backgroundColor: '#fff',
                                    border: '2px solid #e5e7eb',
                                    borderRadius: '12px',
                                    boxShadow: '0 4px 6px rgba(0,0,0,0.1)',
                                }}
                            />
                            <Legend />
                            <Line type="monotone" dataKey="icms" stroke="#3b82f6" strokeWidth={3} name="ICMS" />
                            <Line type="monotone" dataKey="pis" stroke="#10b981" strokeWidth={3} name="PIS" />
                            <Line type="monotone" dataKey="cofins" stroke="#8b5cf6" strokeWidth={3} name="COFINS" />
                        </LineChart>
                    </ResponsiveContainer>
                </div>

                {/* Distribuição por CFOP */}
                <div className="bg-white rounded-2xl shadow-lg p-6 animate-slide-up" style={{ animationDelay: '500ms' }}>
                    <h3 className="text-xl font-bold text-gray-900 mb-6 flex items-center gap-2">
                        <PieChartIcon className="w-6 h-6 text-purple-600" />
                        Distribuição por CFOP
                    </h3>
                    <ResponsiveContainer width="100%" height={300}>
                        <PieChart>
                            <Pie
                                data={cfopDistribution}
                                cx="50%"
                                cy="50%"
                                labelLine={false}
                                label={({ cfop, percentual }) => `${cfop} (${percentual}%)`}
                                outerRadius={100}
                                fill="#8884d8"
                                dataKey="valor"
                            >
                                {cfopDistribution.map((entry, index) => (
                                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                                ))}
                            </Pie>
                            <Tooltip
                                contentStyle={{
                                    backgroundColor: '#fff',
                                    border: '2px solid #e5e7eb',
                                    borderRadius: '12px',
                                    boxShadow: '0 4px 6px rgba(0,0,0,0.1)',
                                }}
                            />
                        </PieChart>
                    </ResponsiveContainer>
                </div>
            </div>

            {/* Alertas Fiscais */}
            <div className="bg-white rounded-2xl shadow-lg p-6 animate-slide-up" style={{ animationDelay: '600ms' }}>
                <h3 className="text-xl font-bold text-gray-900 mb-6 flex items-center gap-2">
                    <AlertTriangle className="w-6 h-6 text-orange-600" />
                    Alertas e Oportunidades
                </h3>
                <div className="space-y-4">
                    {alerts.map((alert, index) => (
                        <div
                            key={index}
                            className={`group relative p-4 rounded-xl border-2 transition-all duration-300 hover:shadow-lg hover:-translate-y-1
                                ${alert.tipo === 'error' ? 'bg-red-50 border-red-300' : ''}
                                ${alert.tipo === 'warning' ? 'bg-yellow-50 border-yellow-300' : ''}
                                ${alert.tipo === 'info' ? 'bg-blue-50 border-blue-300' : ''}
                            `}
                        >
                            <div className="flex items-start gap-4">
                                <div className={`p-2 rounded-lg
                                    ${alert.tipo === 'error' ? 'bg-red-500' : ''}
                                    ${alert.tipo === 'warning' ? 'bg-yellow-500' : ''}
                                    ${alert.tipo === 'info' ? 'bg-blue-500' : ''}
                                `}>
                                    <AlertTriangle className="w-5 h-5 text-white" />
                                </div>
                                <div className="flex-1">
                                    <h4 className="font-bold text-gray-900 mb-1">{alert.titulo}</h4>
                                    <p className="text-sm text-gray-700 mb-2">{alert.descricao}</p>
                                    {alert.documento && (
                                        <p className="text-xs font-mono text-gray-500 bg-white px-2 py-1 rounded inline-block">
                                            {alert.documento}
                                        </p>
                                    )}
                                </div>
                                <button className="px-4 py-2 bg-white rounded-lg font-medium text-sm hover:bg-gray-100 transition-colors">
                                    Revisar
                                </button>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    )
}
