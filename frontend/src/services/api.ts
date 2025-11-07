import axios from 'axios'

const api = axios.create({
    baseURL: '/api',
    headers: {
        'Content-Type': 'application/json',
    },
})

// Interceptor para logging de requisições
api.interceptors.request.use((config) => {
    console.log('📤 Request:', config.method?.toUpperCase(), config.url)
    return config
})

// Interceptor para logging de respostas
api.interceptors.response.use(
    (response) => {
        console.log('📥 Response:', response.status, response.config.url)
        return response
    },
    (error) => {
        console.error('❌ Error:', error.response?.status, error.config?.url)
        return Promise.reject(error)
    }
)

export interface ValidationError {
    code: string
    field: string
    message: string
    severity: 'Critical' | 'High' | 'Medium' | 'Low'
}

export interface ValidationWarning {
    code: string
    field: string
    message: string
    impact: string
}

export interface ValidationResult {
    is_valid: boolean
    errors: ValidationError[]
    warnings: ValidationWarning[]
    suggestions: string[]
}

export interface ProcessResult {
    document_type: string
    chave_acesso: string
    success: boolean
    message: string
    validation?: ValidationResult
    duplicate: boolean
}

export interface DocumentSummary {
    document_type: string
    chave_acesso: string
    numero: string
    serie: string
    data_emissao: string
    emitente: string
    destinatario: string
    valor_total: number
}

export interface StatsResponse {
    total_documents: number
    processed_today: number
    notas_fiscais: number
    ctes: number
}

export const documentService = {
    // Upload e processar XML
    uploadFile: async (file: File): Promise<ProcessResult> => {
        const formData = new FormData()
        formData.append('file', file)
        const { data } = await api.post<ProcessResult>('/documents/upload', formData, {
            headers: { 'Content-Type': 'multipart/form-data' },
        })
        return data
    },

    // Listar documentos
    listDocuments: async (opts?: { doc_type?: 'NFe' | 'CTe'; limit?: number; offset?: number }) => {
        const params: Record<string, any> = {}
        if (opts?.doc_type) params.doc_type = opts.doc_type
        if (opts?.limit) params.limit = opts.limit
        if (opts?.offset) params.offset = opts.offset
        const { data } = await api.get<DocumentSummary[]>('/documents', { params })
        return data
    },

    // Buscar por chave de acesso
    getByChave: async (chave: string) => {
        const { data } = await api.get(`/documents/${chave}`)
        return data
    },

    // Estatísticas
    getStats: async (): Promise<StatsResponse> => {
        const { data } = await api.get<StatsResponse>('/stats')
        return data
    },

    // Exportar dados
    exportData: async (formato: 'json' | 'csv') => {
        const { data } = await api.get(`/export?format=${formato}`, {
            responseType: 'blob',
        })
        return data
    },
}

export default api
