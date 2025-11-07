import { create } from 'zustand'

export interface Document {
    id: string
    tipo: 'NFe' | 'CTe'
    chave_acesso: string
    numero: string
    serie: string
    data_emissao: string
    emitente: string
    destinatario: string
    valor_total: number
    status: 'Completed' | 'Processing' | 'Failed'
}

interface Stats {
    totalDocuments: number
    processedToday: number
    notasFiscais: number
    ctes: number
}

interface Store {
    documents: Document[]
    stats: Stats
    isLoading: boolean
    setDocuments: (documents: Document[]) => void
    setStats: (stats: Stats) => void
    setLoading: (isLoading: boolean) => void
}

export const useStore = create<Store>((set) => ({
    documents: [],
    stats: {
        totalDocuments: 0,
        processedToday: 0,
        notasFiscais: 0,
        ctes: 0,
    },
    isLoading: false,
    setDocuments: (documents) => set({ documents }),
    setStats: (stats) => set({ stats }),
    setLoading: (isLoading) => set({ isLoading }),
}))
