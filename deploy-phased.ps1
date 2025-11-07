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

# Phase 1: Deploy ACR
Write-Host "Phase 1: Deployando Azure Container Registry..." -ForegroundColor Yellow
az deployment group create `
    --resource-group $RESOURCE_GROUP `
    --template-file infra/acr-only.bicep `
    --parameters infra/parameters.acr-only.json `
    --name "$DEPLOYMENT_NAME-acr" `
    --verbose

if ($LASTEXITCODE -ne 0) {
    Write-Host "Erro no deploy do ACR" -ForegroundColor Red
    exit 1
}

# Capturar outputs do ACR
$acrOutputs = az deployment group show `
    --resource-group $RESOURCE_GROUP `
    --name "$DEPLOYMENT_NAME-acr" `
    --query properties.outputs `
    --output json | ConvertFrom-Json

$ACR_LOGIN_SERVER = $acrOutputs.containerRegistryLoginServer.value
$ACR_NAME = $acrOutputs.containerRegistryName.value

Write-Host "ACR deployado com sucesso: $ACR_LOGIN_SERVER" -ForegroundColor Green

# Phase 2: Build e push da imagem
Write-Host "Phase 2: Buildando e enviando imagem..." -ForegroundColor Yellow

Write-Host "Buildando imagem Docker..." -ForegroundColor White
docker build -t geolocation:latest .

if ($LASTEXITCODE -ne 0) {
    Write-Host "Erro no build da imagem" -ForegroundColor Red
    exit 1
}

Write-Host "Fazendo login no ACR..." -ForegroundColor White
az acr login --name $ACR_NAME

if ($LASTEXITCODE -ne 0) {
    Write-Host "Erro no login do ACR" -ForegroundColor Red
    exit 1
}

Write-Host "Taggeando imagem..." -ForegroundColor White
docker tag geolocation:latest "$ACR_LOGIN_SERVER/geolocation:latest"

Write-Host "Enviando imagem para ACR..." -ForegroundColor White
docker push "$ACR_LOGIN_SERVER/geolocation:latest"

if ($LASTEXITCODE -ne 0) {
    Write-Host "Erro no push da imagem" -ForegroundColor Red
    exit 1
}

# Phase 3: Deploy Container App
Write-Host "Phase 3: Deployando Container App..." -ForegroundColor Yellow
az deployment group create `
    --resource-group $RESOURCE_GROUP `
    --template-file infra/main-container-app.bicep `
    --parameters infra/parameters.container-app.json `
    --parameters mongodbUri="$env:MONGODB_URI" `
    --parameters googleMapsApiKey="$env:GOOGLE_MAPS_API_KEY" `
    --name "$DEPLOYMENT_NAME-app" `
    --verbose

if ($LASTEXITCODE -ne 0) {
    Write-Host "Erro no deploy do Container App" -ForegroundColor Red
    exit 1
}

# Capturar outputs finais
$appOutputs = az deployment group show `
    --resource-group $RESOURCE_GROUP `
    --name "$DEPLOYMENT_NAME-app" `
    --query properties.outputs `
    --output json | ConvertFrom-Json

$CONTAINER_APP_URL = $appOutputs.containerAppUrl.value
$CONTAINER_APP_NAME = $appOutputs.containerAppName.value

Write-Host "Deploy completo!" -ForegroundColor Green
Write-Host "Outputs:" -ForegroundColor Cyan
Write-Host "   ACR: $ACR_LOGIN_SERVER" -ForegroundColor White
Write-Host "   Container App: $CONTAINER_APP_NAME" -ForegroundColor White
Write-Host "   URL: $CONTAINER_APP_URL" -ForegroundColor White