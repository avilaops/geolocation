// Stub gerado automaticamente para desenvolvimento sem build WASM.
// Após rodar `build-wasm.ps1`, este arquivo será substituído pelos artefatos wasm-pack.
// Funções retornam estrutura simulada ou lançam erro para indicar ausência do WASM real.

export default async function init() {
    // No stub, nada a inicializar.
    console.warn('[WASM STUB] Módulo WASM real não carregado. Rode build-wasm.ps1 para gerar artefatos.');
}

function fakeResult(tipo: string) {
    return {
        success: true,
        document_type: tipo,
        chave_acesso: 'STUB-000',
        numero: '0',
        serie: '0',
        data_emissao: new Date().toISOString(),
        emitente_nome: 'Emitente Stub',
        emitente_cnpj: '00000000000000',
        destinatario_nome: 'Destinatário Stub',
        destinatario_cnpj: '00000000000000',
        valor_total: 0,
    };
}

export function parse_document_wasm(_xml: string) {
    return fakeResult('Unknown');
}
export function parse_nfe_wasm(_xml: string) {
    return fakeResult('NFe');
}
export function parse_cte_wasm(_xml: string) {
    return fakeResult('CTe');
}
