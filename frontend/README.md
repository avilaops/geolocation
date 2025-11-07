# Frontend - Geolocation

Interface web moderna construída com **React**, **TypeScript** e **Tailwind CSS** para gerenciar documentos fiscais eletrônicos (NF-e e CT-e).

## 🚀 Tecnologias

- **React 18.2** - Biblioteca JavaScript para interfaces
- **TypeScript 5.2** - Superset tipado do JavaScript
- **Vite 5.0** - Build tool ultra-rápido com HMR
- **Tailwind CSS 3.3** - Framework CSS utility-first
- **React Router 6.20** - Roteamento SPA
- **Zustand 4.4** - Gerenciamento de estado
- **Axios** - Cliente HTTP
- **Recharts** - Gráficos e visualizações
- **React Dropzone** - Upload de arquivos drag-and-drop
- **React Hot Toast** - Notificações toast
- **Lucide React** - Ícones modernos

## 📁 Estrutura do Projeto

```
frontend/
├── src/
│   ├── components/         # Componentes reutilizáveis
│   │   ├── Header.tsx      # Cabeçalho com busca e stats
│   │   ├── Layout.tsx      # Layout principal
│   │   ├── Sidebar.tsx     # Menu lateral de navegação
│   │   └── StatCard.tsx    # Card de estatística
│   ├── pages/              # Páginas da aplicação
│   │   ├── Dashboard.tsx   # Dashboard com gráficos
│   │   ├── Upload.tsx      # Upload de XMLs
│   │   ├── NotasFiscais.tsx      # Listagem de NF-e
│   │   ├── ConhecimentosTransporte.tsx  # Listagem de CT-e
│   │   └── Settings.tsx    # Configurações
│   ├── services/           # Serviços e API
│   │   └── api.ts          # Cliente Axios e endpoints
│   ├── store/              # Estado global
│   │   └── useStore.ts     # Store Zustand
│   ├── App.tsx             # Componente raiz com rotas
│   ├── main.tsx            # Entry point
│   └── index.css           # Estilos globais + Tailwind
├── index.html              # Template HTML
├── package.json            # Dependências e scripts
├── tsconfig.json           # Configuração TypeScript
├── vite.config.ts          # Configuração Vite
└── tailwind.config.js      # Configuração Tailwind
```

## 🎨 Características

### 🏠 **Dashboard**
- Cards com estatísticas (total de docs, processados hoje, NF-e, CT-e)
- Gráficos de linha e barra (tendências mensais)
- Tabela de documentos recentes
- Atualização em tempo real via API

### 📤 **Upload**
- Drag-and-drop de arquivos XML
- Upload em lote (batch)
- Feedback visual de progresso
- Validação de tipo de arquivo
- Notificações de sucesso/erro

### 📋 **Listagens (NF-e e CT-e)**
- Tabelas responsivas com dados completos
- Busca por chave de acesso, emitente, destinatário
- Filtros por tipo de documento
- Paginação (quando implementado no backend)
- Exportação de dados

### ⚙️ **Configurações**
- Configuração do banco de dados (SQLite/PostgreSQL)
- Configuração do servidor backend
- Informações de armazenamento
- Opções de processamento automático

## 🔧 Instalação

### Pré-requisitos
- **Node.js** ≥ 18.0.0
- **npm** ≥ 9.0.0 ou **yarn** ≥ 1.22.0

### Passos

1. **Navegue para o diretório frontend**:
```powershell
cd frontend
```

2. **Instale as dependências**:
```powershell
npm install
```

3. **Configure o proxy para o backend** (já está configurado em `vite.config.ts`):
```typescript
server: {
  port: 3000,
  proxy: {
    '/api': 'http://localhost:8080'  // Backend Rust
  }
}
```

## 🚀 Executando

### Modo Desenvolvimento
Inicia o servidor de desenvolvimento com Hot Module Replacement (HMR):
```powershell
npm run dev
```
Acesse: **http://localhost:3000**

### Build para Produção
Gera os arquivos otimizados na pasta `dist/`:
```powershell
npm run build
```

### Preview da Build
Visualiza a build de produção localmente:
```powershell
npm run preview
```

### Linting
Executa o ESLint para verificar o código:
```powershell
npm run lint
```

## 🌐 API Integration

O frontend se comunica com o backend Rust via REST API através do módulo `src/services/api.ts`:

### Endpoints Utilizados

| Método | Endpoint | Descrição |
|--------|----------|-----------|
| `POST` | `/api/documents/upload` | Upload de arquivo XML |
| `GET` | `/api/documents` | Listar todos os documentos |
| `GET` | `/api/documents/:chave` | Buscar por chave de acesso |
| `GET` | `/api/stats` | Estatísticas gerais |
| `GET` | `/api/export?format=json\|csv` | Exportar dados |

### Exemplo de Uso

```typescript
import { documentService } from '@/services/api'

// Upload de arquivo
const file = new File([...], 'nfe.xml')
const result = await documentService.uploadFile(file)

// Listar documentos
const docs = await documentService.listDocuments('NFe')

// Buscar por chave
const doc = await documentService.getByChave('35210812345...')

// Estatísticas
const stats = await documentService.getStats()
```

## 🎨 Customização do Tailwind

O tema personalizado está configurado em `tailwind.config.js`:

```javascript
colors: {
  primary: {
    50: '#f0f9ff',
    100: '#e0f2fe',
    // ... até 900
  }
},
animation: {
  'slide-in': 'slide-in 0.3s ease-out',
  'fade-in': 'fade-in 0.2s ease-out',
}
```

Classes personalizadas no `index.css`:
- `.btn-primary` - Botão primário com gradiente
- `.btn-secondary` - Botão secundário
- `.card` - Card com sombra e bordas arredondadas
- `.input-field` - Campo de input estilizado

## 📦 Estado Global (Zustand)

O store gerencia o estado da aplicação:

```typescript
interface Store {
  documents: Document[]        // Lista de documentos
  stats: Stats                 // Estatísticas
  isLoading: boolean          // Estado de carregamento
  setDocuments: (docs) => void
  setStats: (stats) => void
  setLoading: (loading) => void
}
```

Uso nos componentes:
```typescript
import { useStore } from '@/store/useStore'

const { documents, setDocuments } = useStore()
```

## 🔐 Segurança

- **TypeScript strict mode** - Tipagem rigorosa
- **ESLint** - Análise estática de código
- **CORS** - Configurado no backend para aceitar requests da porta 3000
- **Validação de uploads** - Aceita apenas arquivos `.xml`

## 🐛 Troubleshooting

### Erro de conexão com o backend
- Certifique-se de que o backend Rust está rodando na porta **8080**
- Verifique o proxy em `vite.config.ts`

### Erros de módulo não encontrado
```powershell
rm -rf node_modules package-lock.json
npm install
```

### Conflitos de porta
Altere a porta em `vite.config.ts`:
```typescript
server: {
  port: 3001  // Nova porta
}
```

## 📊 Performance

- **Code splitting automático** - Vite otimiza o bundle
- **Lazy loading** - Páginas carregadas sob demanda
- **Tree shaking** - Remove código não utilizado
- **Minificação** - Código compactado em produção
- **CSS Purge** - Tailwind remove classes não utilizadas

## 🚀 Próximos Passos

- [ ] Adicionar testes unitários (Vitest + React Testing Library)
- [ ] Implementar paginação nas listagens
- [ ] Adicionar filtros avançados (data, valor, status)
- [ ] Modal de detalhes do documento
- [ ] Dark mode
- [ ] PWA (Progressive Web App)
- [ ] Internacionalização (i18n)
- [ ] Gráficos interativos com drill-down

## 📝 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](../LICENSE) para mais detalhes.
