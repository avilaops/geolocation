# Guia de Contribuição - Geolocation Project

Obrigado por considerar contribuir com o projeto Geolocation! ??

Este documento fornece diretrizes e processos para contribuir com o projeto de forma eficiente e padronizada.

## ?? Índice

- [Código de Conduta](#código-de-conduta)
- [Como Posso Contribuir?](#como-posso-contribuir)
- [Configuração do Ambiente](#configuração-do-ambiente)
- [Workflow de Desenvolvimento](#workflow-de-desenvolvimento)
- [Padrões de Código](#padrões-de-código)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Reportando Bugs](#reportando-bugs)
- [Sugerindo Melhorias](#sugerindo-melhorias)

---

## ?? Código de Conduta

Este projeto adere aos princípios de:
- **Respeito mútuo** entre todos os contribuidores
- **Comunicação construtiva** e profissional
- **Inclusão** de pessoas de todas as origens
- **Foco no projeto** e seus objetivos técnicos

---

## ?? Como Posso Contribuir?

### Tipos de Contribuição

- ?? **Correção de Bugs** - Resolver issues existentes
- ? **Novas Features** - Implementar funcionalidades planejadas
- ?? **Documentação** - Melhorar docs, comentários, exemplos
- ?? **Testes** - Adicionar/melhorar cobertura de testes
- ? **Performance** - Otimizações e melhorias de performance
- ?? **UI/UX** - Melhorias visuais no frontend
- ?? **DevOps** - Melhorias em CI/CD, infra, deploy

---

## ??? Configuração do Ambiente

### Pré-requisitos

**Obrigatórios:**
```bash
# Backend
- Rust 1.70+ (rustup)
- Cargo

# Frontend
- Node.js 20+
- npm 10+

# Infraestrutura
- Docker 24+
- Azure CLI 2.50+
- Git 2.40+
```

**Opcional:**
```bash
- PostgreSQL 16+ (para dev local)
- MongoDB 7+ (alternativa)
- Make (para comandos automatizados)
```

### Setup Rápido

#### 1. Clone o Repositório

```bash
git clone https://github.com/avilaops/geolocation.git
cd geolocation
```

#### 2. Setup Automatizado

**Linux/macOS:**
```bash
chmod +x scripts/setup.sh
./scripts/setup.sh
```

**Windows:**
```powershell
.\scripts\setup.ps1
```

**Ou manualmente:**

#### 3. Backend (Rust)

```bash
# Instalar Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar dependências e buildar
cargo build

# Executar testes
cargo test

# Rodar servidor de dev
cargo run --bin geolocation-server
```

#### 4. Frontend (React)

```bash
cd frontend

# Instalar dependências
npm ci

# Dev server (com hot reload)
npm run dev

# Build de produção
npm run build

# Lint
npm run lint
```

#### 5. Ambiente Docker Local

```bash
# Subir todos os serviços (app + banco)
docker-compose -f docker-compose.dev.yml up

# Ou usar Make
make dev
```

#### 6. Variáveis de Ambiente

```bash
# Copiar template
cp .env.example .env

# Editar com suas configurações
nano .env  # ou seu editor preferido
```

**Mínimo necessário para dev:**
```env
DATABASE_URL=sqlite:///app/data/geolocation.db
RUST_LOG=debug
PORT=8080
```

---

## ?? Workflow de Desenvolvimento

### 1. Criar uma Branch

```bash
# Atualizar main
git checkout main
git pull origin main

# Criar branch feature/fix
git checkout -b feature/nome-da-feature
# ou
git checkout -b fix/nome-do-bug
```

**Convenção de Nomes:**
- `feature/` - Novas funcionalidades
- `fix/` - Correção de bugs
- `docs/` - Apenas documentação
- `refactor/` - Refatoração de código
- `test/` - Adição/melhoria de testes
- `perf/` - Melhorias de performance
- `chore/` - Tarefas de manutenção

### 2. Desenvolver

```bash
# Backend: testes contínuos
cargo watch -x test -x check

# Frontend: dev server
npm run dev --prefix frontend

# Verificar padrões
cargo clippy
cargo fmt --check
```

### 3. Commitar

Siga o [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git add .
git commit -m "feat: adicionar endpoint de geocodificação reversa"
```

**Formato:**
```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

**Types:**
- `feat`: Nova feature
- `fix`: Correção de bug
- `docs`: Documentação
- `style`: Formatação (não muda lógica)
- `refactor`: Refatoração
- `perf`: Melhoria de performance
- `test`: Testes
- `chore`: Manutenção/build

**Exemplos:**
```bash
feat(api): adicionar endpoint POST /api/geocode
fix(parser): corrigir parsing de XML com encoding UTF-8
docs(readme): atualizar instruções de instalação
refactor(database): migrar de SQLite para PostgreSQL
test(api): adicionar testes de integração para rotas
perf(xml): otimizar parser com processamento paralelo
```

### 4. Push e Pull Request

```bash
# Push da branch
git push origin feature/nome-da-feature

# Abrir PR no GitHub
# Use o template automático que aparecerá
```

---

## ?? Padrões de Código

### Rust (Backend)

#### Formatação

```bash
# Auto-format (obrigatório antes de commit)
cargo fmt

# Verificar warnings
cargo clippy -- -D warnings
```

#### Estilo

```rust
// ? BOM: Nomes descritivos, snake_case
fn process_nfe_xml(file_path: &Path) -> Result<NfeDocument, Error> {
    // ...
}

// ? RUIM: Nomes genéricos, camelCase
fn procXML(fp: &Path) -> Result<Doc, Error> {
    // ...
}

// ? BOM: Documentação clara
/// Processa arquivo XML de Nota Fiscal Eletrônica (NF-e).
///
/// # Arguments
/// * `file_path` - Caminho do arquivo XML
///
/// # Returns
/// * `Ok(NfeDocument)` - Documento processado com sucesso
/// * `Err(Error)` - Erro de parsing ou validação
///
/// # Examples
/// ```rust
/// let doc = process_nfe_xml(Path::new("nfe.xml"))?;
/// ```
pub fn process_nfe_xml(file_path: &Path) -> Result<NfeDocument, Error> {
    // implementação
}

// ? BOM: Error handling explícito
let data = parse_xml(&content)
    .map_err(|e| Error::ParseError(e))?;

// ? RUIM: Usar .unwrap() em código de produção
let data = parse_xml(&content).unwrap();  // EVITAR!
```

#### Testes

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geocode_success() {
        // Arrange
    let address = "Rua Exemplo, 123";
        
        // Act
     let result = geocode(address);
  
        // Assert
      assert!(result.is_ok());
  let coords = result.unwrap();
    assert!(coords.lat >= -90.0 && coords.lat <= 90.0);
    }

    #[tokio::test]
    async fn test_api_health_check() {
        let response = call_health_endpoint().await;
        assert_eq!(response.status(), 200);
    }
}
```

### TypeScript/React (Frontend)

#### Formatação

```bash
# Lint e fix
npm run lint --prefix frontend
```

#### Componentes

```typescript
// ? BOM: Functional component com TypeScript
import { FC } from 'react';

interface DocumentListProps {
  documents: Document[];
  onSelect: (doc: Document) => void;
}

export const DocumentList: FC<DocumentListProps> = ({ 
  documents, 
  onSelect 
}) => {
  return (
    <div className="document-list">
      {documents.map(doc => (
     <DocumentCard 
        key={doc.id}
          document={doc}
          onClick={() => onSelect(doc)}
  />
      ))}
    </div>
  );
};

// ? BOM: Custom hooks
export const useDocuments = () => {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchDocuments = async () => {
    setLoading(true);
    try {
      const data = await api.getDocuments();
      setDocuments(data);
    } catch (error) {
      toast.error('Erro ao carregar documentos');
    } finally {
      setLoading(false);
    }
  };

  return { documents, loading, fetchDocuments };
};
```

### SQL/Migrações

```sql
-- migrations/001_create_documents.sql
-- ? BOM: Comentários descritivos, idempotente

-- Criar tabela de documentos fiscais
CREATE TABLE IF NOT EXISTS documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_type VARCHAR(10) NOT NULL,  -- 'NFE' ou 'CTE'
    xml_content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Índices para performance
CREATE INDEX IF NOT EXISTS idx_documents_type ON documents(document_type);
CREATE INDEX IF NOT EXISTS idx_documents_created ON documents(created_at DESC);
```

---

## ?? Commit Guidelines

### Estrutura

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Exemplos Completos

```bash
# Feature simples
feat(api): adicionar endpoint de busca por CNPJ

# Feature com descrição
feat(geocoding): implementar cache de geocodificação

Adiciona cache em memória (LRU) para resultados de
geocodificação, reduzindo chamadas à API do Google Maps.

- Cache configurável via variável GEOCODE_CACHE_SIZE
- TTL padrão de 1 hora
- Testes unitários adicionados

Refs: #123

# Bugfix
fix(parser): corrigir parse de NFe com múltiplos itens

O parser falhava quando NFe continha mais de 100 itens.
Ajustado para processar arrays grandes corretamente.

Fixes: #456

# Breaking change
feat(api)!: alterar formato de resposta da API

BREAKING CHANGE: O endpoint /api/documents agora retorna
objeto paginado ao invés de array direto.

Antes:
{
  "documents": [...]
}

Depois:
{
  "data": [...],
  "page": 1,
  "total": 100
}

Migration guide: docs/migration-v2.md
```

---

## ?? Pull Request Process

### Checklist Antes de Abrir PR

- [ ] Código compila sem erros
- [ ] Testes passando (`cargo test` e `npm test`)
- [ ] Código formatado (`cargo fmt`, `npm run lint`)
- [ ] Sem warnings do Clippy (`cargo clippy`)
- [ ] Documentação atualizada (se aplicável)
- [ ] Testes adicionados para nova funcionalidade
- [ ] CHANGELOG.md atualizado
- [ ] Branch atualizada com `main`

### Template de PR

Use o template automático `.github/PULL_REQUEST_TEMPLATE.md`:

```markdown
## Descrição
[Descrição clara do que foi alterado]

## Tipo de Mudança
- [ ] Bugfix
- [ ] Nova feature
- [ ] Breaking change
- [ ] Documentação

## Como Testar
1. [Passo a passo para testar]

## Checklist
- [ ] Testes passando
- [ ] Código formatado
- [ ] Documentação atualizada

## Screenshots (se aplicável)
[Prints de tela]
```

### Review Process

1. **Automated Checks** - CI/CD deve passar
2. **Code Review** - Pelo menos 1 aprovação
3. **Testing** - Testar localmente se necessário
4. **Merge** - Squash merge preferencial

---

## ?? Reportando Bugs

### Antes de Reportar

1. Verifique [Issues existentes](https://github.com/avilaops/geolocation/issues)
2. Teste na versão mais recente
3. Tente reproduzir consistentemente

### Template de Bug Report

Use `.github/ISSUE_TEMPLATE/bug_report.md`:

```markdown
**Descrição do Bug**
Descrição clara do problema.

**Para Reproduzir**
1. Vá para '...'
2. Clique em '...'
3. Veja erro

**Comportamento Esperado**
O que deveria acontecer.

**Screenshots**
Se aplicável.

**Ambiente:**
 - OS: [Windows/Linux/macOS]
 - Rust version: [1.70]
 - Node version: [20.0]
 - Browser: [Chrome 120]

**Logs**
```
[cole logs relevantes]
```
```

---

## ?? Sugerindo Melhorias

### Feature Requests

Use `.github/ISSUE_TEMPLATE/feature_request.md`:

```markdown
**Problema a Resolver**
Descrição do problema ou necessidade.

**Solução Proposta**
Como você resolveria.

**Alternativas Consideradas**
Outras abordagens possíveis.

**Contexto Adicional**
Qualquer contexto relevante.
```

---

## ?? Release Process

### Versionamento (SemVer)

```
MAJOR.MINOR.PATCH

1.2.3
? ? ?? Bugfixes
? ???? Novas features (backward compatible)
?????? Breaking changes
```

### Criar Release

```bash
# 1. Atualizar CHANGELOG.md
# 2. Commit
git commit -m "chore: release v1.2.0"

# 3. Tag
git tag -a v1.2.0 -m "Release v1.2.0"

# 4. Push
git push origin main --tags
```

---

## ?? Recursos Adicionais

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Docs](https://docs.rs/axum/)
- [React Docs](https://react.dev/)
- [Azure Bicep](https://learn.microsoft.com/azure/azure-resource-manager/bicep/)
- [Conventional Commits](https://www.conventionalcommits.org/)

---

## ?? Para Agentes Copilot

### Contexto Essencial

```yaml
# Projeto
Nome: Geolocation
Objetivo: Processamento de NF-e/CT-e com geocodificação
Linguagens: Rust (backend) + TypeScript (frontend)

# Arquitetura
Backend: Axum web framework, SQLx, Tokio async runtime
Frontend: React 18, Vite, TailwindCSS, Zustand
Database: SQLite (dev), PostgreSQL (prod)
Deploy: Azure App Service + ACR

# Comandos Chave
Build Backend: cargo build --release
Test Backend: cargo test
Run Server: cargo run --bin geolocation-server
Build Frontend: npm run build --prefix frontend
Deploy: ./deploy-appservice.ps1 -Environment prod

# Estrutura
src/           - Backend Rust
frontend/      - Frontend React
infra/         - IaC Bicep
.github/       - CI/CD workflows
```

### Padrões Importantes

- **Error Handling**: Sempre usar `Result<T, Error>`, nunca `.unwrap()` em prod
- **Async**: Todas operações I/O devem ser async com Tokio
- **Testing**: Mínimo 70% cobertura para novo código
- **Docs**: Documentar toda função pública com `///`
- **Commits**: Conventional Commits obrigatório
- **CI/CD**: GitHub Actions valida tudo antes de merge

---

**Dúvidas?** Abra uma [Discussion](https://github.com/avilaops/geolocation/discussions) ou entre em contato com os mantenedores.

**Obrigado por contribuir! ??**
