# Módulo WASM

Este diretório recebe os artefatos gerados por `wasm-pack build` (fora de `src` para evitar poluir código-fonte).

## Como gerar

No diretório raiz do projeto:

```powershell
./build-wasm.ps1
```

Isso irá:

1. Instalar `wasm-pack` (se necessário)
2. Gerar os arquivos compilados para WebAssembly dentro de `frontend/wasm`
3. Compilar o backend em release

## Arquivos esperados após build

- `geolocation_bg.wasm`
- `geolocation.js` (ou `geolocation.ts` dependendo de configuração)
- `package.json` (metadados wasm-pack)

## Stub temporário

O diretório `frontend/src/wasm` contém um stub TypeScript (`geolocation.ts`) que permite executar o frontend durante desenvolvimento sem compilar WASM. Após o build real, o alias `@wasm` no Vite apontará para este diretório (`frontend/wasm`) onde estarão os artefatos reais.

## Configuração Vite

O `vite.config.ts` está configurado com alias `@wasm` para `./wasm`, facilitando imports:

```typescript
import wasmModule from '@wasm/geolocation'
```

## Erros comuns

- Erro Vite: `Failed to resolve import '@wasm/geolocation'`
  - Significa que o build WASM não foi executado. Execute `./build-wasm.ps1` primeiro.
- Cache do Vite: reinicie `npm run dev` após o build.

## Dica

Durante desenvolvimento, o hook `useWasm` carrega o stub automaticamente. Você verá um indicador visual na interface mostrando se está usando stub ou WASM real.
