#!/bin/bash
# ===================================================================
# Setup Script - Geolocation Project
# ===================================================================
# Este script automatiza a configuração inicial do ambiente de dev
#
# Uso: ./scripts/setup.sh

set -euo pipefail

# Cores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "========================================"
echo "  Geolocation Project - Setup Script  "
echo "========================================"
echo -e "${NC}"

# ===================================================================
# Funções Utilitárias
# ===================================================================

print_step() {
    echo -e "${BLUE}▶ $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# ===================================================================
# Verificar Dependências
# ===================================================================

print_step "Verificando dependências..."

# Rust
if ! command_exists rustc; then
    print_error "Rust não encontrado!"
    echo "Instale com: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
else
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    print_success "Rust $RUST_VERSION encontrado"
fi

# Cargo
if ! command_exists cargo; then
    print_error "Cargo não encontrado!"
    exit 1
else
    CARGO_VERSION=$(cargo --version | awk '{print $2}')
    print_success "Cargo $CARGO_VERSION encontrado"
fi

# Node.js
if ! command_exists node; then
    print_warning "Node.js não encontrado. Frontend não será configurado."
    SKIP_FRONTEND=true
else
    NODE_VERSION=$(node --version)
  print_success "Node.js $NODE_VERSION encontrado"
    SKIP_FRONTEND=false
fi

# npm
if ! command_exists npm && [ "$SKIP_FRONTEND" = false ]; then
    print_warning "npm não encontrado"
    SKIP_FRONTEND=true
else
    NPM_VERSION=$(npm --version)
    print_success "npm $NPM_VERSION encontrado"
fi

# Docker (opcional)
if command_exists docker; then
    DOCKER_VERSION=$(docker --version | awk '{print $3}' | tr -d ',')
    print_success "Docker $DOCKER_VERSION encontrado"
    HAS_DOCKER=true
else
    print_warning "Docker não encontrado (opcional)"
    HAS_DOCKER=false
fi

# Git
if ! command_exists git; then
    print_error "Git não encontrado!"
    exit 1
else
    GIT_VERSION=$(git --version | awk '{print $3}')
    print_success "Git $GIT_VERSION encontrado"
fi

echo ""

# ===================================================================
# Instalar Ferramentas Rust
# ===================================================================

print_step "Instalando ferramentas Rust..."

# cargo-watch (hot reload)
if ! command_exists cargo-watch; then
    print_step "Instalando cargo-watch..."
    cargo install cargo-watch
    print_success "cargo-watch instalado"
else
    print_success "cargo-watch já instalado"
fi

# sqlx-cli (migrations)
if ! command_exists sqlx; then
    print_step "Instalando sqlx-cli..."
    cargo install sqlx-cli --no-default-features --features postgres,sqlite
    print_success "sqlx-cli instalado"
else
    print_success "sqlx-cli já instalado"
fi

# cargo-audit (security)
if ! cargo audit --version >/dev/null 2>&1; then
    print_step "Instalando cargo-audit..."
    cargo install cargo-audit
    print_success "cargo-audit instalado"
else
    print_success "cargo-audit já instalado"
fi

echo ""

# ===================================================================
# Setup Backend (Rust)
# ===================================================================

print_step "Configurando backend Rust..."

# Fetch dependencies
print_step "Baixando dependências Rust..."
cargo fetch
print_success "Dependências baixadas"

# Build (debug mode)
print_step "Compilando projeto (debug)..."
cargo build
print_success "Build concluído"

echo ""

# ===================================================================
# Setup Frontend (React)
# ===================================================================

if [ "$SKIP_FRONTEND" = false ]; then
    print_step "Configurando frontend React..."
    
 cd frontend
  
    # Instalar dependências
    print_step "Instalando dependências npm..."
    npm ci
    print_success "Dependências instaladas"
    
    # Build
    print_step "Compilando frontend..."
    npm run build
    print_success "Build concluído"
    
    cd ..
    echo ""
else
    print_warning "Frontend ignorado (Node.js não disponível)"
    echo ""
fi

# ===================================================================
# Setup Banco de Dados
# ===================================================================

print_step "Configurando banco de dados..."

# Criar diretório de dados
mkdir -p data
print_success "Diretório data/ criado"

# Verificar se DATABASE_URL está setado
if [ -z "${DATABASE_URL:-}" ]; then
  print_warning "DATABASE_URL não configurado"
    echo "Usando SQLite padrão: sqlite:///app/data/geolocation.db"
    export DATABASE_URL="sqlite://data/geolocation.db"
fi

# Rodar migrations
if command_exists sqlx; then
    print_step "Executando migrations..."
    if [ -d "migrations" ]; then
        sqlx database create || true
        sqlx migrate run
        print_success "Migrations executadas"
    else
    print_warning "Pasta migrations/ não encontrada"
    fi
else
    print_warning "sqlx-cli não instalado, migrations ignoradas"
fi

echo ""

# ===================================================================
# Setup Variáveis de Ambiente
# ===================================================================

print_step "Configurando variáveis de ambiente..."

if [ ! -f ".env" ]; then
    if [ -f ".env.example" ]; then
        cp .env.example .env
        print_success ".env criado a partir de .env.example"
        print_warning "⚠ Edite .env e configure suas credenciais!"
    else
        print_warning ".env.example não encontrado"
    fi
else
    print_success ".env já existe"
fi

echo ""

# ===================================================================
# Git Hooks (opcional)
# ===================================================================

print_step "Configurando Git hooks..."

if [ -d ".git" ]; then
    # Pre-commit hook
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook
echo "Running pre-commit checks..."

# Format check
cargo fmt --check || {
    echo "Code not formatted. Run: cargo fmt"
    exit 1
}

# Clippy
cargo clippy -- -D warnings || {
    echo "Clippy warnings found"
    exit 1
}

echo "✓ Pre-commit checks passed"
EOF

    chmod +x .git/hooks/pre-commit
    print_success "Git hooks configurados"
else
    print_warning "Não é um repositório Git, hooks ignorados"
fi

echo ""

# ===================================================================
# Docker Setup (opcional)
# ===================================================================

if [ "$HAS_DOCKER" = true ]; then
    print_step "Docker detectado"
    echo "Para usar Docker Compose:"
echo "  make dev         # ou docker-compose -f docker-compose.dev.yml up"
    echo ""
fi

# ===================================================================
# Testes
# ===================================================================

print_step "Executando testes..."

if cargo test --quiet; then
    print_success "Testes passaram"
else
    print_warning "Alguns testes falharam (normal em setup inicial)"
fi

echo ""

# ===================================================================
# Resumo Final
# ===================================================================

echo -e "${GREEN}"
echo "========================================"
echo "  ✓ Setup Concluído com Sucesso!"
echo "========================================"
echo -e "${NC}"

echo "Próximos passos:"
echo ""
echo "1. Configure .env com suas credenciais:"
echo "   ${BLUE}nano .env${NC}"
echo ""
echo "2. Inicie o servidor de desenvolvimento:"
echo "   ${BLUE}make dev${NC}  # ou: cargo run --bin geolocation-server"
echo ""
echo "3. (Opcional) Frontend dev server:"
echo "   ${BLUE}make dev-frontend${NC}  # ou: npm run dev --prefix frontend"
echo ""
echo "4. Acesse a aplicação:"
echo "   ${BLUE}http://localhost:8080${NC}"
echo ""
echo "5. Veja comandos disponíveis:"
echo "   ${BLUE}make help${NC}"
echo ""

echo "Documentação:"
echo "  - CONTRIBUTING.md  - Guia de contribuição"
echo "  - ARCHITECTURE.md  - Documentação de arquitetura"
echo "  - README-DEPLOY.md - Guia de deploy"
echo ""

echo -e "${GREEN}Happy coding! 🚀${NC}"
