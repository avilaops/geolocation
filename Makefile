# Makefile para Geolocation Project
# Comandos padronizados para desenvolvimento, teste e deploy
#
# Uso: make [target]
# Lista todos os targets: make help

.PHONY: help install build test clean run dev deploy docker-up docker-down lint format check

# Variáveis
CARGO := cargo
NPM := npm
DOCKER := docker
DOCKER_COMPOSE := docker-compose -f docker-compose.dev.yml
FRONTEND_DIR := frontend
DEPLOYMENT_ENV := dev

# Cores para output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m # No Color

##@ Geral

help: ## Mostra esta mensagem de ajuda
	@echo "$(BLUE)Geolocation Project - Make Commands$(NC)"
	@echo ""
@awk 'BEGIN {FS = ":.*##"; printf "Uso:\n  make $(CYAN)<target>$(NC)\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(CYAN)%-20s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n$(YELLOW)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Instalação & Setup

install: install-backend install-frontend ## Instala todas as dependências (backend + frontend)
	@echo "$(GREEN)? Todas as dependências instaladas!$(NC)"

install-backend: ## Instala dependências do Rust
	@echo "$(BLUE)Installing Rust dependencies...$(NC)"
	$(CARGO) fetch
	$(CARGO) build

install-frontend: ## Instala dependências do frontend
	@echo "$(BLUE)Installing frontend dependencies...$(NC)"
	cd $(FRONTEND_DIR) && $(NPM) ci

setup: install ## Alias para 'install'
	@echo "$(GREEN)? Setup concluído!$(NC)"

##@ Build

build: build-backend build-frontend ## Build completo (backend + frontend)
	@echo "$(GREEN)? Build concluído!$(NC)"

build-backend: ## Build do backend Rust (debug)
	@echo "$(BLUE)Building Rust backend (debug)...$(NC)"
	$(CARGO) build

build-backend-release: ## Build do backend Rust (release/otimizado)
	@echo "$(BLUE)Building Rust backend (release)...$(NC)"
	$(CARGO) build --release

build-frontend: ## Build do frontend React
	@echo "$(BLUE)Building React frontend...$(NC)"
	cd $(FRONTEND_DIR) && $(NPM) run build

build-docker: ## Build da imagem Docker
	@echo "$(BLUE)Building Docker image...$(NC)"
	$(DOCKER) build -t geolocation:latest .

##@ Testes

test: test-backend test-frontend ## Executa todos os testes
	@echo "$(GREEN)? Todos os testes passaram!$(NC)"

test-backend: ## Executa testes do Rust
	@echo "$(BLUE)Running Rust tests...$(NC)"
	$(CARGO) test

test-backend-coverage: ## Executa testes com cobertura (requer tarpaulin)
	@echo "$(BLUE)Running tests with coverage...$(NC)"
	$(CARGO) tarpaulin --out Html --output-dir coverage

test-frontend: ## Executa testes do frontend
	@echo "$(BLUE)Running frontend tests...$(NC)"
	cd $(FRONTEND_DIR) && $(NPM) test

test-watch: ## Executa testes em modo watch (auto-reload)
	@echo "$(BLUE)Running tests in watch mode...$(NC)"
	$(CARGO) watch -x test

test-integration: ## Executa apenas testes de integração
	@echo "$(BLUE)Running integration tests...$(NC)"
	$(CARGO) test --test '*'

##@ Desenvolvimento

dev: ## Inicia ambiente de desenvolvimento completo (Docker Compose)
	@echo "$(BLUE)Starting dev environment...$(NC)"
	$(DOCKER_COMPOSE) up

dev-detach: ## Inicia dev environment em background
	@echo "$(BLUE)Starting dev environment (detached)...$(NC)"
	$(DOCKER_COMPOSE) up -d

dev-backend: ## Inicia apenas backend em modo dev
	@echo "$(BLUE)Starting Rust dev server...$(NC)"
	$(CARGO) run --bin geolocation-server

dev-frontend: ## Inicia apenas frontend em modo dev
	@echo "$(BLUE)Starting Vite dev server...$(NC)"
	cd $(FRONTEND_DIR) && $(NPM) run dev

dev-watch: ## Dev server com hot reload (requer cargo-watch)
	@echo "$(BLUE)Starting with hot reload...$(NC)"
	$(CARGO) watch -x 'run --bin geolocation-server'

run: ## Alias para 'dev-backend'
	$(MAKE) dev-backend

##@ Code Quality

lint: lint-backend lint-frontend ## Executa linters (backend + frontend)
	@echo "$(GREEN)? Lint completo!$(NC)"

lint-backend: ## Executa Clippy (linter Rust)
	@echo "$(BLUE)Running Clippy...$(NC)"
	$(CARGO) clippy -- -D warnings

lint-frontend: ## Executa ESLint
	@echo "$(BLUE)Running ESLint...$(NC)"
	cd $(FRONTEND_DIR) && $(NPM) run lint

format: format-backend format-frontend ## Formata código (backend + frontend)
	@echo "$(GREEN)? Código formatado!$(NC)"

format-backend: ## Formata código Rust com rustfmt
	@echo "$(BLUE)Formatting Rust code...$(NC)"
	$(CARGO) fmt

format-frontend: ## Formata código TypeScript/React com Prettier
	@echo "$(BLUE)Formatting frontend code...$(NC)"
	cd $(FRONTEND_DIR) && $(NPM) run format || echo "$(YELLOW)? npm format script not found$(NC)"

check: ## Verifica código sem compilar (rápido)
	@echo "$(BLUE)Checking code...$(NC)"
	$(CARGO) check

check-all: check lint test ## Executa todas as verificações (check + lint + test)
	@echo "$(GREEN)? Todas as verificações passaram!$(NC)"

##@ Docker

docker-up: ## Sobe containers Docker (dev)
	@echo "$(BLUE)Starting Docker containers...$(NC)"
	$(DOCKER_COMPOSE) up -d

docker-down: ## Para e remove containers Docker
	@echo "$(BLUE)Stopping Docker containers...$(NC)"
	$(DOCKER_COMPOSE) down

docker-logs: ## Mostra logs dos containers
	$(DOCKER_COMPOSE) logs -f

docker-ps: ## Lista containers em execução
	$(DOCKER_COMPOSE) ps

docker-rebuild: ## Rebuild e reinicia containers
	@echo "$(BLUE)Rebuilding containers...$(NC)"
	$(DOCKER_COMPOSE) up --build -d

docker-clean: ## Remove containers, volumes e imagens
	@echo "$(RED)Cleaning Docker resources...$(NC)"
	$(DOCKER_COMPOSE) down -v --rmi local

docker-shell: ## Abre shell no container da aplicação
	$(DOCKER_COMPOSE) exec app sh

docker-db-shell: ## Abre psql no container PostgreSQL
	$(DOCKER_COMPOSE) exec postgres psql -U geolocation

##@ Database

db-migrate: ## Executa migrations do banco de dados
	@echo "$(BLUE)Running database migrations...$(NC)"
	$(CARGO) sqlx migrate run

db-migrate-redo: ## Desfaz última migration e roda novamente
	@echo "$(BLUE)Redoing last migration...$(NC)"
	$(CARGO) sqlx migrate revert
	$(CARGO) sqlx migrate run

db-reset: ## Reseta banco de dados (? CUIDADO: apaga dados!)
	@echo "$(RED)Resetting database...$(NC)"
	rm -f data/*.db
	$(CARGO) sqlx migrate run

db-seed: ## Popula banco com dados de teste
	@echo "$(BLUE)Seeding database...$(NC)"
	$(CARGO) run --bin seed-db || echo "$(YELLOW)? Seed script not found$(NC)"

##@ Deploy

deploy-dev: ## Deploy para ambiente dev (Azure)
	@echo "$(BLUE)Deploying to dev environment...$(NC)"
	./deploy-appservice.ps1 -Environment dev

deploy-prod: ## Deploy para produção (Azure)
	@echo "$(YELLOW)? Deploying to PRODUCTION...$(NC)"
	./deploy-appservice.ps1 -Environment prod

deploy-infra-only: ## Deploy apenas infraestrutura (sem app)
	@echo "$(BLUE)Deploying infrastructure only...$(NC)"
	./deploy-appservice.ps1 -Environment $(DEPLOYMENT_ENV) -InfraOnly

##@ Limpeza

clean: clean-backend clean-frontend ## Limpa arquivos de build
	@echo "$(GREEN)? Limpeza concluída!$(NC)"

clean-backend: ## Limpa build artifacts do Rust
	@echo "$(BLUE)Cleaning Rust build...$(NC)"
	$(CARGO) clean

clean-frontend: ## Limpa build artifacts do frontend
	@echo "$(BLUE)Cleaning frontend build...$(NC)"
	rm -rf $(FRONTEND_DIR)/dist
	rm -rf $(FRONTEND_DIR)/node_modules/.vite

clean-all: clean docker-clean ## Limpa tudo (build + Docker)
	@echo "$(GREEN)? Limpeza completa concluída!$(NC)"

##@ Utilitários

logs: ## Mostra logs da aplicação
	tail -f logs/*.log || echo "$(YELLOW)No log files found$(NC)"

docs: ## Gera documentação do código Rust
	@echo "$(BLUE)Generating Rust docs...$(NC)"
	$(CARGO) doc --no-deps --open

bench: ## Executa benchmarks (requer criterion)
	@echo "$(BLUE)Running benchmarks...$(NC)"
	$(CARGO) bench

outdated: ## Lista dependências desatualizadas
	@echo "$(BLUE)Checking outdated dependencies...$(NC)"
	$(CARGO) outdated
	cd $(FRONTEND_DIR) && $(NPM) outdated

security-audit: ## Verifica vulnerabilidades de segurança
	@echo "$(BLUE)Running security audit...$(NC)"
	$(CARGO) audit
	cd $(FRONTEND_DIR) && $(NPM) audit

update: ## Atualiza dependências
	@echo "$(BLUE)Updating dependencies...$(NC)"
	$(CARGO) update
	cd $(FRONTEND_DIR) && $(NPM) update

version: ## Mostra versões das ferramentas
	@echo "$(BLUE)Tool versions:$(NC)"
	@echo "Rust:   $$($(CARGO) --version)"
	@echo "Node:   $$(node --version)"
	@echo "npm:    $$($(NPM) --version)"
	@echo "Docker: $$($(DOCKER) --version)"

##@ CI/CD

ci: check-all build ## Simula pipeline de CI localmente
	@echo "$(GREEN)? CI pipeline completo!$(NC)"

pre-commit: format lint test ## Hook pre-commit (roda antes de commit)
	@echo "$(GREEN)? Pre-commit checks passaram!$(NC)"

pre-push: check-all ## Hook pre-push (roda antes de push)
	@echo "$(GREEN)? Pre-push checks passaram!$(NC)"

# Aliases comuns
.PHONY: up down start stop restart

up: docker-up ## Alias para docker-up
down: docker-down ## Alias para docker-down
start: dev ## Alias para dev
stop: docker-down ## Alias para docker-down
restart: docker-down docker-up ## Reinicia containers Docker

# Default target
.DEFAULT_GOAL := help
