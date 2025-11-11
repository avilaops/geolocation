# Changelog

Todas as mudanças notáveis neste projeto serão documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
e este projeto adere ao [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Documentação completa do projeto (CONTRIBUTING.md, ARCHITECTURE.md)
- Templates GitHub para issues e PRs
- Infraestrutura como código (Bicep) para Azure
- CI/CD via GitHub Actions
- Docker Compose para desenvolvimento local
- Makefile com comandos padronizados
- Scripts de setup automatizado
- .editorconfig para consistência de código

### Changed
- Migração de ARM JSON para Bicep
- Otimização do Dockerfile multi-stage

### Security
- Configuração de Azure Managed Identity
- Integração com Key Vault para secrets

## [0.1.0] - 2025-01-11

### Added
- Servidor web Rust com Axum
- Parser de XML para NF-e/CT-e
- Integração com Google Maps API (geocoding, routing)
- Frontend React com TypeScript e Vite
- Suporte multi-database (SQLite, PostgreSQL, MongoDB)
- Monitoring com Prometheus e tracing
- WebAssembly bindings
- Healthcheck endpoint
- Logging estruturado

### Backend
- Processamento assíncrono com Tokio
- Type-safe database queries com SQLx
- Error handling robusto com thiserror
- Cache em memória para geocoding

### Frontend
- Dashboard para visualização de documentos
- Upload de arquivos XML via drag & drop
- Gráficos e relatórios com Recharts
- State management com Zustand
- Notificações com React Hot Toast
- Estilização com TailwindCSS

### Infrastructure
- Azure Container Registry
- Azure App Service (Linux + Docker)
- Application Insights
- Log Analytics Workspace
- PostgreSQL Flexible Server (opcional)
- Key Vault (opcional)

---

## Tipos de Mudanças

- `Added` - Novas funcionalidades
- `Changed` - Mudanças em funcionalidades existentes
- `Deprecated` - Funcionalidades que serão removidas em breve
- `Removed` - Funcionalidades removidas
- `Fixed` - Correções de bugs
- `Security` - Melhorias de segurança

---

## Links

- [Unreleased]: https://github.com/avilaops/geolocation/compare/v0.1.0...HEAD
- [0.1.0]: https://github.com/avilaops/geolocation/releases/tag/v0.1.0
