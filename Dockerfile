# Dockerfile multi-stage para build otimizado

# ============================================================
# Stage 1: Build do Frontend (TEMPORARIAMENTE DESABILITADO)
# ============================================================
# FROM node:20-alpine AS frontend-builder
# 
# WORKDIR /app/frontend
# 
# # Copiar apenas package files para cache de dependências
# COPY frontend/package*.json ./
# RUN npm ci
# 
# # Copiar código fonte e buildar
# COPY frontend/ ./
# RUN npm run build

# ============================================================
# Stage 2: Build do Backend (Rust)
# ============================================================
FROM rust:latest AS backend-builder

# Instalar dependências de sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar Cargo files para cache de dependências
COPY Cargo.toml build.rs ./
COPY src/ ./src/

# Build em release mode
RUN cargo build --release --bin geolocation-server

# ============================================================
# Stage 3: Imagem final (runtime leve)
# ============================================================
FROM debian:bookworm-slim

# Instalar apenas dependências de runtime
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binário compilado do backend
COPY --from=backend-builder /app/target/release/geolocation-server ./geolocation-server

# Copiar frontend buildado (TEMPORARIAMENTE DESABILITADO)
# COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

# Variáveis de ambiente (atualizadas para MongoDB)
ENV RUST_LOG=info
ENV PORT=8080

# Expor porta
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Executar servidor
CMD ["./geolocation-server"]
