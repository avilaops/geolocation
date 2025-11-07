import { useEffect, useState, useCallback } from 'react'
import { documentService, DocumentSummary } from '../services/api'
import { ChevronLeft, ChevronRight, Filter } from 'lucide-react'
import toast from 'react-hot-toast'

interface DocumentTableProps {
    pageSize?: number
    initialType?: 'ALL' | 'NFe' | 'CTe'
}

export default function DocumentTable({ pageSize = 20, initialType = 'ALL' }: DocumentTableProps) {
    const [items, setItems] = useState<DocumentSummary[]>([])
    const [loading, setLoading] = useState(false)
    const [error, setError] = useState<string | null>(null)
    const [docType, setDocType] = useState<'ALL' | 'NFe' | 'CTe'>(initialType)
    const [page, setPage] = useState(0)
    const [total, setTotal] = useState<number>(0)

    const load = useCallback(async () => {
        setLoading(true)
        setError(null)
        try {
            // Usa stats para total aproximado (sem endpoint dedicado de count por filtro)
            const stats = await documentService.getStats()
            const totalDocs = docType === 'NFe' ? stats.notas_fiscais : docType === 'CTe' ? stats.ctes : stats.total_documents
            setTotal(totalDocs)
            const data = await documentService.listDocuments({ doc_type: docType === 'ALL' ? undefined : docType, limit: pageSize, offset: page * pageSize })
            setItems(data)
        } catch (e: any) {
            setError(e.message || 'Erro ao listar documentos')
            toast.error('Falha ao carregar documentos')
        } finally {
            setLoading(false)
        }
    }, [docType, page, pageSize])

    useEffect(() => {
        load()
    }, [load])

    const totalPages = Math.max(1, Math.ceil(total / pageSize))

    const changeType = (t: 'ALL' | 'NFe' | 'CTe') => {
        setDocType(t)
        setPage(0)
    }

    return (
        <div className="space-y-4">
            <div className="flex items-center justify-between">
                <h3 className="text-lg font-semibold text-gray-900 flex items-center gap-2">
                    <Filter className="w-4 h-4 text-indigo-500" /> Explorar Documentos
                </h3>
                <div className="flex items-center gap-2">
                    {(['ALL', 'NFe', 'CTe'] as const).map(t => (
                        <button
                            key={t}
                            onClick={() => changeType(t)}
                            className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-all ${docType === t ? 'bg-indigo-600 text-white shadow' : 'bg-gray-100 text-gray-700 hover:bg-gray-200'}`}
                        >
                            {t === 'ALL' ? 'Todos' : t}
                        </button>
                    ))}
                </div>
            </div>

            <div className="bg-white rounded-xl border border-gray-200 overflow-hidden shadow-sm">
                <div className="overflow-x-auto">
                    <table className="w-full text-sm">
                        <thead className="bg-gray-50 text-gray-600 uppercase text-xs font-medium">
                            <tr>
                                <th className="px-4 py-3 text-left">Tipo</th>
                                <th className="px-4 py-3 text-left">Chave</th>
                                <th className="px-4 py-3 text-left">Número</th>
                                <th className="px-4 py-3 text-left">Emitente</th>
                                <th className="px-4 py-3 text-left">Destinatário</th>
                                <th className="px-4 py-3 text-left">Data Emissão</th>
                                <th className="px-4 py-3 text-right">Valor Total</th>
                            </tr>
                        </thead>
                        <tbody className="divide-y divide-gray-200">
                            {loading && (
                                <tr>
                                    <td colSpan={7} className="px-4 py-6 text-center text-indigo-600 font-medium">Carregando...</td>
                                </tr>
                            )}
                            {!loading && items.length === 0 && (
                                <tr>
                                    <td colSpan={7} className="px-4 py-6 text-center text-gray-500">Nenhum documento encontrado</td>
                                </tr>
                            )}
                            {!loading && items.map(doc => (
                                <tr key={doc.chave_acesso} className="hover:bg-gray-50">
                                    <td className="px-4 py-2">
                                        <span className={`px-2 py-1 rounded-full text-xs font-semibold ${doc.document_type === 'NFe' ? 'bg-purple-100 text-purple-700' : 'bg-orange-100 text-orange-700'}`}>{doc.document_type}</span>
                                    </td>
                                    <td className="px-4 py-2 font-mono text-xs text-gray-600">{doc.chave_acesso.slice(0, 22)}...</td>
                                    <td className="px-4 py-2 text-gray-700">{doc.numero}</td>
                                    <td className="px-4 py-2 text-gray-900 truncate max-w-[160px]" title={doc.emitente}>{doc.emitente}</td>
                                    <td className="px-4 py-2 text-gray-700 truncate max-w-[160px]" title={doc.destinatario}>{doc.destinatario}</td>
                                    <td className="px-4 py-2 text-gray-600">{new Date(doc.data_emissao).toLocaleDateString('pt-BR')}</td>
                                    <td className="px-4 py-2 text-right font-medium text-gray-900">{doc.valor_total.toLocaleString('pt-BR', { style: 'currency', currency: 'BRL' })}</td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
                <div className="flex items-center justify-between px-4 py-3 bg-gray-50 border-t border-gray-200">
                    <div className="text-xs text-gray-600">Página {page + 1} de {totalPages} • {total} registros</div>
                    <div className="flex items-center gap-2">
                        <button
                            disabled={page === 0 || loading}
                            onClick={() => setPage(p => Math.max(0, p - 1))}
                            className="p-2 rounded-md bg-white border border-gray-300 disabled:opacity-40 hover:bg-gray-100"
                            title="Anterior"
                        >
                            <ChevronLeft className="w-4 h-4" />
                        </button>
                        <button
                            disabled={page + 1 >= totalPages || loading}
                            onClick={() => setPage(p => (p + 1 < totalPages ? p + 1 : p))}
                            className="p-2 rounded-md bg-white border border-gray-300 disabled:opacity-40 hover:bg-gray-100"
                            title="Próxima"
                        >
                            <ChevronRight className="w-4 h-4" />
                        </button>
                    </div>
                </div>
            </div>

            {error && (
                <div className="px-4 py-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-700">{error}</div>
            )}
        </div>
    )
}
