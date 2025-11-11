## ?? Descrição

<!-- Descrição clara e concisa das mudanças -->

## ?? Tipo de Mudança

<!-- Marque as opções relevantes -->

- [ ] ?? Bugfix (correção de bug)
- [ ] ? Feature (nova funcionalidade)
- [ ] ?? Breaking change (mudança que quebra compatibilidade)
- [ ] ?? Documentação
- [ ] ?? Refatoração (melhoria de código sem mudar comportamento)
- [ ] ? Performance (melhoria de performance)
- [ ] ? Testes (adição ou correção de testes)
- [ ] ?? Chore (manutenção, build, CI/CD)

## ?? Issues Relacionadas

<!-- Link para issues que este PR resolve -->

Fixes #(issue_number)
Relates to #(issue_number)

## ?? Como Testar

<!-- Passos detalhados para testar as mudanças -->

1. Faça checkout desta branch: `git checkout feature/branch-name`
2. Instale dependências: `cargo build && npm ci --prefix frontend`
3. Execute testes: `cargo test && npm test --prefix frontend`
4. Teste manualmente:
   - [ ] Passo 1
   - [ ] Passo 2
   - [ ] Passo 3

**Comandos úteis:**
```bash
# Backend
cargo run --bin geolocation-server

# Frontend
npm run dev --prefix frontend
```

## ? Checklist

### Código

- [ ] Código compila sem erros (`cargo build`, `npm run build`)
- [ ] Testes passando (`cargo test`, `npm test`)
- [ ] Código formatado (`cargo fmt`, `npm run lint`)
- [ ] Sem warnings do Clippy (`cargo clippy -- -D warnings`)
- [ ] Sem warnings de ESLint

### Documentação

- [ ] Comentários adicionados/atualizados em código complexo
- [ ] Documentação técnica atualizada (se aplicável)
- [ ] README.md atualizado (se aplicável)
- [ ] CHANGELOG.md atualizado

### Testes

- [ ] Testes unitários adicionados para novo código
- [ ] Testes de integração adicionados (se aplicável)
- [ ] Cobertura de testes mantida ou aumentada
- [ ] Casos edge testados

### Outros

- [ ] Branch atualizada com `main` (sem conflitos)
- [ ] Commits seguem [Conventional Commits](https://www.conventionalcommits.org/)
- [ ] Mudanças não introduzem vulnerabilidades de segurança
- [ ] Performance não foi degradada

## ?? Screenshots/Demos (se aplicável)

<!-- Para mudanças visuais no frontend -->

**Antes:**
<!-- Screenshot ou descrição do estado anterior -->

**Depois:**
<!-- Screenshot ou descrição do novo estado -->

## ?? Mudanças Detalhadas

### Backend (Rust)

<!-- Liste arquivos/módulos modificados no backend -->

- `src/module.rs`: Descrição da mudança
- `src/another.rs`: Descrição da mudança

### Frontend (React/TypeScript)

<!-- Liste componentes/páginas modificadas no frontend -->

- `frontend/src/components/Component.tsx`: Descrição da mudança
- `frontend/src/pages/Page.tsx`: Descrição da mudança

### Infraestrutura

<!-- Se aplicável, mudanças em Bicep, Docker, CI/CD -->

- `infra/main.bicep`: Descrição da mudança
- `.github/workflows/deploy.yml`: Descrição da mudança

## ?? Breaking Changes (se aplicável)

<!-- Descreva mudanças que quebram compatibilidade -->

**Antes:**
```rust
// código antigo
```

**Depois:**
```rust
// código novo
```

**Migration Guide:**
<!-- Instruções para migrar código existente -->

## ?? Deployment Notes (se aplicável)

<!-- Instruções especiais para deploy -->

- [ ] Requer migrations de banco: `sqlx migrate run`
- [ ] Requer novas variáveis de ambiente
- [ ] Requer restart do serviço
- [ ] Requer update de secrets no Key Vault

**Variáveis de Ambiente Novas:**
```env
NEW_VARIABLE=value
```

## ?? Performance Impact (se aplicável)

<!-- Métricas de performance antes/depois -->

**Antes:**
- Tempo de resposta: X ms
- Uso de memória: Y MB

**Depois:**
- Tempo de resposta: X ms (-Z%)
- Uso de memória: Y MB (-Z%)

## ?? Para Reviewers

<!-- Pontos específicos onde você gostaria de feedback -->

- [ ] Lógica de negócio no módulo X
- [ ] Escolha de estrutura de dados em Y
- [ ] Arquitetura da feature Z
- [ ] Nomenclatura de funções/variáveis

## ?? Notas Adicionais

<!-- Qualquer informação extra relevante para o review -->
