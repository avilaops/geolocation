import { useEffect, useState } from 'react'

interface WasmModule {
    parse_document_wasm: (xmlContent: string) => any
    parse_nfe_wasm: (xmlContent: string) => any
    parse_cte_wasm: (xmlContent: string) => any
    __isStub?: boolean
}

interface WasmParseResult {
    success: boolean
    document_type: string
    chave_acesso: string
    numero: string
    serie: string
    data_emissao: string
    emitente_nome: string
    emitente_cnpj: string
    destinatario_nome: string
    destinatario_cnpj: string
    valor_total: number
    error?: string
}

export function useWasm() {
    const [wasm, setWasm] = useState<WasmModule | null>(null)
    const [loading, setLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)

    useEffect(() => {
        async function loadWasm() {
            try {
                // Tenta carregar artefato wasm real (gerado pelo wasm-pack via alias @wasm)
                // Se falhar, carrega stub de src/wasm
                let wasmModule: any
                let isStub = false

                try {
                    wasmModule = await import('@wasm/geolocation')
                    console.log('✅ WASM real carregado de frontend/wasm')
                } catch {
                    wasmModule = await import('../wasm/geolocation')
                    isStub = true
                    console.warn('⚠️ Usando WASM stub - Execute build-wasm.ps1 para módulo real')
                }

                if (typeof wasmModule.default === 'function') {
                    await wasmModule.default()
                }
                setWasm({ ...wasmModule, __isStub: isStub } as WasmModule)
                setLoading(false)
            } catch (err) {
                console.error('❌ Falha ao carregar módulo WASM:', err)
                setError(err instanceof Error ? err.message : 'Erro desconhecido')
                setLoading(false)
            }
        }

        loadWasm()
    }, [])

    const parseDocument = (xmlContent: string): WasmParseResult | null => {
        if (!wasm) {
            console.error('WASM module not loaded')
            return null
        }

        try {
            return wasm.parse_document_wasm(xmlContent)
        } catch (err) {
            console.error('Error parsing document:', err)
            return {
                success: false,
                document_type: 'Unknown',
                chave_acesso: '',
                numero: '',
                serie: '',
                data_emissao: '',
                emitente_nome: '',
                emitente_cnpj: '',
                destinatario_nome: '',
                destinatario_cnpj: '',
                valor_total: 0,
                error: err instanceof Error ? err.message : 'Unknown error',
            }
        }
    }

    const parseNFe = (xmlContent: string): WasmParseResult | null => {
        if (!wasm) {
            console.error('WASM module not loaded')
            return null
        }

        try {
            return wasm.parse_nfe_wasm(xmlContent)
        } catch (err) {
            console.error('Error parsing NFe:', err)
            return null
        }
    }

    const parseCTe = (xmlContent: string): WasmParseResult | null => {
        if (!wasm) {
            console.error('WASM module not loaded')
            return null
        }

        try {
            return wasm.parse_cte_wasm(xmlContent)
        } catch (err) {
            console.error('Error parsing CTe:', err)
            return null
        }
    }

    return {
        wasm,
        loading,
        error,
        parseDocument,
        parseNFe,
        parseCTe,
    }
}
