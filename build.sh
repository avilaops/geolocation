#!/bin/bash
# Script de compilação para Linux/macOS
# Compila o projeto com otimizações máximas

set -e

echo "Compilando Geolocation..."

# Verifica se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "Erro: Rust não está instalado!"
    echo "Instale via: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Verifica se o NASM está instalado
if ! command -v nasm &> /dev/null; then
    echo "Aviso: NASM não encontrado. As otimizações Assembly não serão compiladas."
    echo "Instale via: sudo apt install nasm (Ubuntu/Debian) ou brew install nasm (macOS)"
fi

# Compilação em modo release
echo "Compilando em modo release..."
RUSTFLAGS="-C target-cpu=native" cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "Compilação concluída com sucesso!"
    echo "Executável em: target/release/geolocation"
    
    # Mostra o tamanho do executável
    size=$(du -h target/release/geolocation | cut -f1)
    echo "Tamanho: $size"
    
    # Torna o executável... executável
    chmod +x target/release/geolocation
else
    echo ""
    echo "Erro na compilação!"
    exit 1
fi
