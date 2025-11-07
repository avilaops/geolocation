import { useState, useEffect } from 'react'
import { Search, Truck } from 'lucide-react'
import { useStore, Document } from '../store/useStore'
import { documentService } from '../services/api'
import toast from 'react-hot-toast'

export default function ConhecimentosTransporte() {
    const { documents, setDocuments } = useStore()
    const [searchTerm, setSearchTerm] = useState('')
    const [filteredDocs, setFilteredDocs] = useState<Document[]>([])

    useEffect(() => {
        loadDocuments()
    }, [])

    useEffect(() => {
        const ctes = documents.filter((doc) => doc.tipo === 'CTe')
        if (searchTerm) {
            const filtered = ctes.filter(
                (doc) =>
                    doc.chave_acesso.toLowerCase().includes(searchTerm.toLowerCase()) ||
                    doc.emitente.toLowerCase().includes(searchTerm.toLowerCase()) ||
                    doc.destinatario.toLowerCase().includes(searchTerm.toLowerCase())
            )
            setFilteredDocs(filtered)
        } else {
            setFilteredDocs(ctes)
        }
    }, [documents, searchTerm])

    const loadDocuments = async () => {
        try {
            const data = await documentService.listDocuments('CTe')
            setDocuments(data)
        } catch (error) {
            console.error('Erro ao carregar conhecimentos de transporte:', error)
            toast.error('Erro ao carregar conhecimentos de transporte')

            // Dados de exemplo
            setDocuments([
                {
                    id: '1',
                    tipo: 'CTe',
                    chave_acesso: '35210812345678901234567890123456789012345678',
                    numero: '000045',
                    serie: '1',
                    data_emissao: '2024-01-15T14:20:00',
                    emitente: 'Transportadora Rápida Ltda',
                    destinatario: 'Cliente Final SA',
                    valor_total: 850.00,
                    status: 'Completed',
                },
            ])
        }
    }

    return (
        <div className="space-y-6">
            {/* Título */}
            <div className="flex items-center justify-between">
                <div>
                    <h1 className="text-3xl font-bold text-gray-900">
                        Conhecimentos de Transporte (CT-e)
                    </h1>
                    <p className="text-gray-600 mt-1">
                        {filteredDocs.length} documento(s) encontrado(s)
                    </p>
                </div>
                <button className="btn-primary">
                    Exportar Dados
                </button>
            </div>

            {/* Busca */}
            <div className="card">
                <div className="relative">
                    <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
                    <input
                        type="text"
                        placeholder="Buscar por chave de acesso, transportadora ou destinatário..."
                        value={searchTerm}
                        onChange={(e) => setSearchTerm(e.target.value)}
                        className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
                    />
                </div>
            </div>

            {/* Tabela */}
            <div className="card overflow-hidden">
                <div className="overflow-x-auto">
                    <table className="w-full">
                        <thead className="bg-gray-50 border-b border-gray-200">
                            <tr>
                                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Número
                                </th>
                                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Chave de Acesso
                                </th>
                                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Transportadora
                                </th>
                                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Destinatário
                                </th>
                                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                                    Data
                                </th>
                                <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                                    Valor Total
                                </th>
                            </tr>
                        </thead>
                        <tbody className="divide-y divide-gray-200">
                            {filteredDocs.length === 0 ? (
                                <tr>
                                    <td colSpan={6} className="px-4 py-12 text-center">
                                        <Truck className="w-12 h-12 mx-auto text-gray-300 mb-2" />
                                        <p className="text-gray-500">Nenhum conhecimento de transporte encontrado</p>
                                        <p className="text-sm text-gray-400 mt-1">
                                            Faça upload de arquivos XML para começar
                                        </p>
                                    </td>
                                </tr>
                            ) : (
                                filteredDocs.map((doc) => (
                                    <tr key={doc.id} className="hover:bg-gray-50 cursor-pointer">
                                        <td className="px-4 py-3 whitespace-nowrap">
                                            <span className="text-sm font-medium text-gray-900">
                                                {doc.numero}/{doc.serie}
                                            </span>
                                        </td>
                                        <td className="px-4 py-3">
                                            <span className="text-xs font-mono text-gray-600">
                                                {doc.chave_acesso}
                                            </span>
                                        </td>
                                        <td className="px-4 py-3 text-sm text-gray-900">
                                            {doc.emitente}
                                        </td>
                                        <td className="px-4 py-3 text-sm text-gray-900">
                                            {doc.destinatario}
                                        </td>
                                        <td className="px-4 py-3 text-sm text-gray-600 whitespace-nowrap">
                                            {new Date(doc.data_emissao).toLocaleDateString('pt-BR')}
                                        </td>
                                        <td className="px-4 py-3 text-sm text-right font-medium text-gray-900 whitespace-nowrap">
                                            {doc.valor_total.toLocaleString('pt-BR', {
                                                style: 'currency',
                                                currency: 'BRL',
                                            })}
                                        </td>
                                    </tr>
                                ))
                            )}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    )
}
