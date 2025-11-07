#!/usr/bin/env pwsh

$ErrorActionPreference = "Stop"

# Carregar variaveis do .env
Get-Content .env | ForEach-Object {
    if ($_ -match '^([^=]+)=(.*)$') {
        [Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
    }
}

$RESOURCE_GROUP = "shancrys-rg"
$LOCATION = "eastus2"
$DEPLOYMENT_NAME = "geolocation-deploy-$(Get-Date -Format 'yyyyMMddHHmmss')"

Write-Host "Iniciando deploy da infraestrutura..." -ForegroundColor Cyan

# Verificar se o resource group existe
Write-Host "Usando resource group existente: $RESOURCE_GROUP" -ForegroundColor Yellow

# Deploy da infraestrutura
Write-Host "Deployando infraestrutura via Bicep..." -ForegroundColor Yellow
az deployment group create `
    --resource-group $RESOURCE_GROUP `
    --template-file infra/main-container-app.bicep `
    --parameters infra/parameters.container-app.json `
    --parameters mongodbUri="$env:MONGODB_URI" `
    --parameters googleMapsApiKey="$env:GOOGLE_MAPS_API_KEY" `
    --name $DEPLOYMENT_NAME `
    --verbose

if ($LASTEXITCODE -eq 0) {
    Write-Host "Infraestrutura deployada com sucesso!" -ForegroundColor Green
    
    # Capturar outputs
    $outputs = az deployment group show `
        --resource-group $RESOURCE_GROUP `
        --name $DEPLOYMENT_NAME `
        --query properties.outputs `
        --output json | ConvertFrom-Json
    
    $ACR_LOGIN_SERVER = $outputs.containerRegistryLoginServer.value
    $CONTAINER_APP_NAME = $outputs.containerAppName.value
    $CONTAINER_APP_URL = $outputs.containerAppUrl.value
    
    Write-Host "Outputs:" -ForegroundColor Cyan
    Write-Host "   ACR: $ACR_LOGIN_SERVER" -ForegroundColor White
    Write-Host "   Container App: $CONTAINER_APP_NAME" -ForegroundColor White
    Write-Host "   URL: $CONTAINER_APP_URL" -ForegroundColor White
    
    # Build e push da imagem Docker
    Write-Host "Buildando imagem Docker..." -ForegroundColor Yellow
    docker build -t geolocation:latest .
    
    Write-Host "Fazendo login no ACR..." -ForegroundColor Yellow
    $acrName = $ACR_LOGIN_SERVER.Replace('.azurecr.io', '')
    az acr login --name $acrName
    
    Write-Host "Taggeando imagem..." -ForegroundColor Yellow
    docker tag geolocation:latest "$ACR_LOGIN_SERVER/geolocation:latest"
    
    Write-Host "Enviando imagem para ACR..." -ForegroundColor Yellow
    docker push "$ACR_LOGIN_SERVER/geolocation:latest"
    
    Write-Host "Atualizando Container App..." -ForegroundColor Yellow
    az containerapp update `
        --name $CONTAINER_APP_NAME `
        --resource-group $RESOURCE_GROUP `
        --image "$ACR_LOGIN_SERVER/geolocation:latest"
    
    Write-Host "Deploy completo! App disponivel em: $CONTAINER_APP_URL" -ForegroundColor Green
} else {
    Write-Host "Erro no deploy da infraestrutura" -ForegroundColor Red
    exit 1
}
