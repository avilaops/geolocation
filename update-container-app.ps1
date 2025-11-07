#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

# Carregar variaveis do .env
Get-Content .env | ForEach-Object {
    if ($_ -match '^([^=]+)=(.*)$') {
        [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
    }
}

$RESOURCE_GROUP = "shancrys-rg"
$CONTAINER_APP_NAME = "geodev01-app"
$ACR_LOGIN_SERVER = "acrgeodev01.azurecr.io"
$IMAGE_NAME = "geolocation:latest"

Write-Host "Atualizando Container App com nova imagem..." -ForegroundColor Cyan

# Atualizar Container App com a nova imagem
az containerapp update `
    --name $CONTAINER_APP_NAME `
    --resource-group $RESOURCE_GROUP `
    --image "$ACR_LOGIN_SERVER/$IMAGE_NAME" `
    --set-env-vars `
        "MONGODB_URI=secretref:mongodb-uri" `
        "GOOGLE_MAPS_API_KEY=secretref:google-maps-api-key" `
        "PORT=8080" `
        "RUST_LOG=info"

if ($LASTEXITCODE -eq 0) {
    Write-Host "Container App atualizado com sucesso!" -ForegroundColor Green
    
    # Obter URL da aplicação
    $URL = az containerapp show `
        --name $CONTAINER_APP_NAME `
        --resource-group $RESOURCE_GROUP `
        --query "properties.configuration.ingress.fqdn" `
        --output tsv
    
    if ($URL) {
        Write-Host "Aplicação disponível em: https://$URL" -ForegroundColor Green
    }
} else {
    Write-Host "Erro ao atualizar Container App" -ForegroundColor Red
    exit 1
}
