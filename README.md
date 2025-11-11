# 🌍 Geolocation - Processamento de Documentos Fiscais com Geocodificação

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-18-blue.svg)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-blue.svg)](https://www.typescriptlang.org/)
[![Azure](https://img.shields.io/badge/Azure-Deployed-0078D4.svg)](https://azure.microsoft.com/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

Sistema de alta performance para processamento de **Notas Fiscais Eletrônicas (NF-e)** e **Conhecimentos de Transporte (CT-e)** com capacidades de **geocodificação** e **cálculo de rotas**.

## 📋 Índice

- [Sobre o Projeto](#-sobre-o-projeto)
- [Características](#-características)
- [Stack Tecnológica](#-stack-tecnológica)
- [Quick Start](#-quick-start)
- [Documentação](#-documentação)
- [Desenvolvimento](#-desenvolvimento)
- [Deploy](#-deploy)
- [Contribuindo](#-contribuindo)
- [Licença](#-licença)

---

## 🎯 Sobre o Projeto

O **Geolocation** foi desenvolvido para processar documentos fiscais XML (NF-e/CT-e), extrair informações geográficas, geocodificar endereços e fornecer análises visuais através de uma interface web moderna.

### Casos de Uso

- 📦 **Logística**: Otimização de rotas de transporte
- 📊 **Análise Fiscal**: Visualização geográfica de operações
- 🗺️ **Planejamento**: Mapeamento de fornecedores e clientes
- 🚛 **Gestão de Frotas**: Rastreamento e análise de entregas

---

## ✨ Características

### Backend (Rust)

- ⚡ **Alta Performance**: Processamento XML 5-10x mais rápido que Python/Node
- 🔒 **Type-Safe**: Segurança de tipos em compile-time
- 🌊 **Async/Await**: Processamento assíncrono com Tokio
- 🗄️ **Multi-Database**: SQLite (dev), PostgreSQL/MongoDB (prod)
- 🔍 **Geocoding**: Integração com Google Maps API
- 📊 **Observabilidade**: Prometheus metrics + distributed tracing
- 🧪 **Testado**: Cobertura de testes robusta

### Frontend (React)

- 💎 **Moderno**: React 18 + TypeScript + Vite
- 🎨 **Responsivo**: TailwindCSS para UI elegante
- 📈 **Visualizações**: Gráficos interativos com Recharts
- 🔔 **UX**: Notificações em tempo real
- 📁 **Drag & Drop**: Upload de arquivos intuitivo
- ⚡ **Rápido**: Build otimizado com Vite

### Infraestrutura (Azure)

- 🐳 **Containerizado**: Docker + Azure Container Registry
- ☁️ **Cloud-Native**: Azure App Service + Key Vault
- 📊 **Monitoramento**: Application Insights + Log Analytics
- 🔐 **Seguro**: Managed Identity + RBAC
- 🚀 **CI/CD**: GitHub Actions automático

---

## 🛠️ Stack Tecnológica

| Categoria | Tecnologias |
|-----------|-------------|
| **Backend** | Rust, Axum, SQLx, Tokio, Serde |
| **Frontend** | React, TypeScript, Vite, TailwindCSS, Zustand |
| **Database** | PostgreSQL, SQLite, MongoDB |
| **APIs** | Google Maps (Geocoding, Routing, Distance Matrix) |
| **Cloud** | Azure (ACR, App Service, Key Vault, App Insights) |
| **DevOps** | Docker, GitHub Actions, Bicep (IaC) |
| **Monitoring** | Prometheus, Application Insights, Tracing |

---

## 🚀 Quick Start

### Opção 1: Docker Compose (Recomendado)

```bash
# Clone o repositório
git clone https://github.com/avilaops/geolocation.git
cd geolocation

# Copie variáveis de ambiente
cp .env.example .env

# Suba os containers
docker-compose -f docker-compose.dev.yml up

# Acesse: http://localhost:8080
```

### Opção 2: Instalação Local

```bash
# Pré-requisitos: Rust 1.70+, Node.js 20+, PostgreSQL (opcional)

# Setup automatizado
chmod +x scripts/setup.sh
./scripts/setup.sh

# Ou manual:
make install   # Instala dependências
make build     # Build backend + frontend
make dev  # Inicia dev server
```

### Opção 3: Makefile

```bash
# Ver todos comandos disponíveis
make help

# Comandos úteis
make install      # Instala deps (backend + frontend)
make dev             # Inicia ambiente dev completo
make test         # Executa todos os testes
make check-all       # Verifica code quality
make deploy-dev      # Deploy para Azure (dev)
```

---

## 📚 Documentação

| Documento | Descrição |
|-----------|-----------|
| [CONTRIBUTING.md](CONTRIBUTING.md) | Guia para contribuidores |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Arquitetura detalhada do sistema |
| [README-DEPLOY.md](README-DEPLOY.md) | Guia completo de deploy Azure |
| [CHANGELOG.md](CHANGELOG.md) | Histórico de versões |

### Estrutura do Projeto

```
geolocation/
├── src/                    # Backend Rust
│   ├── main.rs            # CLI
│   ├── server.rs # Web server (Axum)
│   ├── google_maps_api.rs # Cliente Google Maps
│   └── ...
├── frontend/      # Frontend React
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   └── ...
│   └── package.json
├── infra/  # Infraestrutura (Bicep)
│   ├── main.bicep
│   └── parameters.*.json
├── .github/  # CI/CD workflows
├── scripts/# Scripts de automação
├── Dockerfile        # Multi-stage build
├── Makefile    # Comandos padronizados
└── README.md
```

---

## 💻 Desenvolvimento

### Comandos Principais

```bash
# Backend
cargo build       # Build debug
cargo test          # Testes
cargo run --bin geolocation-server# Dev server
cargo clippy           # Linter
cargo fmt     # Formatar código

# Frontend
npm run dev --prefix frontend    # Dev server (Vite)
npm run build --prefix frontend  # Build produção
npm run lint --prefix frontend   # ESLint

# Docker
docker-compose -f docker-compose.dev.yml up     # Sobe ambiente completo
docker-compose -f docker-compose.dev.yml logs -f  # Ver logs

# Database
make db-migrate  # Rodar migrations
make db-seed         # Popular com dados de teste
```

### Endpoints API

```
GET  /api/health  # Health check
POST /api/upload           # Upload XML (NF-e/CT-e)
GET  /api/documents  # Listar documentos
GET  /api/documents/:id       # Detalhes documento
POST /api/geocode     # Geocodificar endereço
POST /api/route    # Calcular rota
GET  /api/search?q=...   # Buscar documentos
```

### Variáveis de Ambiente

```env
# Obrigatórias
DATABASE_URL=postgresql://user:pass@host/db
RUST_LOG=info
PORT=8080

# Opcionais
GOOGLE_MAPS_API_KEY=your_api_key
AZURE_KEY_VAULT_NAME=your_keyvault
```

Veja [.env.example](.env.example) para lista completa.

---

## 🚀 Deploy

### Deploy Automático (GitHub Actions)

1. Configure secrets no GitHub:
   - `AZURE_CLIENT_ID`
   - `AZURE_TENANT_ID`
   - `AZURE_SUBSCRIPTION_ID`

2. Push para `main`:
   ```bash
   git push origin main
   ```

3. GitHub Actions executa automaticamente:
   - ✅ Testes
   - ✅ Build
   - ✅ Deploy infraestrutura (Bicep)
   - ✅ Push imagem para ACR
   - ✅ Atualiza App Service

### Deploy Manual (Azure CLI)

```bash
# Usando script PowerShell
./deploy-appservice.ps1 -Environment dev

# Ou usando Makefile
make deploy-dev
```

Veja [README-DEPLOY.md](README-DEPLOY.md) para guia completo.

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Leia [CONTRIBUTING.md](CONTRIBUTING.md)
2. Fork o projeto
3. Crie uma feature branch: `git checkout -b feature/MinhaFeature`
4. Commit suas mudanças: `git commit -m 'feat: adiciona MinhaFeature'`
5. Push para a branch: `git push origin feature/MinhaFeature`
6. Abra um Pull Request

### Padrões

- **Commits**: Seguir [Conventional Commits](https://www.conventionalcommits.org/)
- **Código**: Rust (rustfmt + clippy), TypeScript (ESLint + Prettier)
- **Testes**: Mínimo 70% cobertura para novo código

---

## 📊 Roadmap

### v0.2.0 (Em desenvolvimento)
- [ ] Autenticação/Autorização (OAuth2)
- [ ] API de busca full-text
- [ ] Cache distribuído (Redis)
- [ ] Rate limiting

### v0.3.0 (Planejado)
- [ ] Processamento em background (queue)
- [ ] Exportação de relatórios (PDF)
- [ ] Webhooks
- [ ] Multi-tenancy

### v1.0.0 (Futuro)
- [ ] Machine Learning (classificação automática)
- [ ] GraphQL API
- [ ] Mobile app (React Native)

---

## 📝 Licença

Este projeto está sob a licença MIT. Veja [LICENSE](LICENSE) para mais detalhes.

---

## 👥 Mantenedores

- **Avila DevOps** - [@avilaops](https://github.com/avilaops)

---

## 🙏 Agradecimentos

- [Rust Community](https://www.rust-lang.org/community)
- [Axum](https://github.com/tokio-rs/axum)
- [React](https://reactjs.org/)
- [Azure](https://azure.microsoft.com/)

---

## 📞 Suporte

- 📧 Email: [suporte@avilaops.com](mailto:suporte@avilaops.com)
- 💬 Discussions: [GitHub Discussions](https://github.com/avilaops/geolocation/discussions)
- 🐛 Issues: [GitHub Issues](https://github.com/avilaops/geolocation/issues)

---

<p align="center">
  Feito com ❤️ por <a href="https://github.com/avilaops">Avila DevOps</a>
</p>

<p align="center">
  <sub>⭐ Dê uma estrela se este projeto foi útil!</sub>
</p>
