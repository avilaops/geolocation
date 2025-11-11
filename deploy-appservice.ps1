# ===================================================================
# Deploy Rápido para Azure App Service - Geolocation
# ===================================================================
# Uso: .\deploy-appservice.ps1 [-Environment dev|prod] [-InfraOnly] [-SkipBuild]

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet('dev','prod')]
 [string]$Environment = 'dev',
    
    [Parameter(Mandatory=$false)]
    [string]$ResourceGroup = 'rg-geolocation',
    
    [Parameter(Mandatory=$false)]
    [string]$Location = 'eastus',
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipBuild,
    
  [Parameter(Mandatory=$false)]
    [switch]$InfraOnly
)

$ErrorActionPreference = 'Stop'

Write-Host ""
Write-Host "?? Geolocation Azure Deploy" -ForegroundColor Cyan
Write-Host "=============================" -ForegroundColor Cyan
Write-Host "Environment: $Environment" -ForegroundColor Yellow
Write-Host "Resource Group: $ResourceGroup" -ForegroundColor Yellow
Write-Host ""

# Verificar Azure CLI
if (!(Get-Command az -ErrorAction SilentlyContinue)) {
    Write-Host "? Azure CLI não encontrado. Instale: https://aka.ms/azure-cli" -ForegroundColor Red
    exit 1
}

# Login check
$account = az account show 2>$null | ConvertFrom-Json
if (!$account) {
    Write-Host "?? Fazendo login no Azure..." -ForegroundColor Yellow
    az login
}

Write-Host "? Logado: $($account.user.name)" -ForegroundColor Green
Write-Host ""

# Criar RG
Write-Host "?? Resource Group..." -ForegroundColor Cyan
az group create --name $ResourceGroup --location $Location --output none
Write-Host "? Resource group pronto" -ForegroundColor Green
Write-Host ""

# Deploy infra
Write-Host "??? Deploy da infraestrutura (Bicep)..." -ForegroundColor Cyan
$deploymentName = "geo-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
$parametersFile = "infra/parameters.$Environment.json"

$deployment = az deployment group create `
    --resource-group $ResourceGroup `
    --name $deploymentName `
    --template-file infra/main.bicep `
    --parameters "@$parametersFile" `
    --parameters resourceToken="${Environment}01" `
 --output json | ConvertFrom-Json

$acrLoginServer = $deployment.properties.outputs.containerRegistryLoginServer.value
$webAppName = $deployment.properties.outputs.appServiceName.value

Write-Host "? Infraestrutura OK" -ForegroundColor Green
Write-Host "   ACR: $acrLoginServer" -ForegroundColor Gray
Write-Host "   App: $webAppName" -ForegroundColor Gray
Write-Host ""

if ($InfraOnly) {
    Write-Host "? Deploy de infraestrutura concluído!" -ForegroundColor Green
    exit 0
}

# Build Docker
if (!$SkipBuild) {
    Write-Host "?? Build Docker..." -ForegroundColor Cyan
    $imageTag = Get-Date -Format "yyyyMMdd-HHmmss"
    docker build -t "geolocation:$imageTag" -t "geolocation:latest" .
    Write-Host "? Imagem criada" -ForegroundColor Green
    Write-Host ""
} else {
    $imageTag = "latest"
}

# Push ACR
Write-Host "?? Push para ACR..." -ForegroundColor Cyan
$acrName = $acrLoginServer -replace '\.azurecr\.io$', ''
az acr login --name $acrName --output none

docker tag "geolocation:$imageTag" "$acrLoginServer/geolocation:$imageTag"
docker push "$acrLoginServer/geolocation:$imageTag"
docker tag "geolocation:$imageTag" "$acrLoginServer/geolocation:latest"
docker push "$acrLoginServer/geolocation:latest"

Write-Host "? Imagem no ACR" -ForegroundColor Green
Write-Host ""

# Config App Service
Write-Host "?? Configurando Web App..." -ForegroundColor Cyan
az webapp config container set `
    --name $webAppName `
    --resource-group $ResourceGroup `
  --docker-custom-image-name "$acrLoginServer/geolocation:$imageTag" `
    --docker-registry-server-url "https://$acrLoginServer" `
    --output none

az webapp restart --name $webAppName --resource-group $ResourceGroup --output none
Write-Host "? App configurado e reiniciado" -ForegroundColor Green
Write-Host ""

# URL final
$webAppUrl = az webapp show --name $webAppName --resource-group $ResourceGroup --query defaultHostName -o tsv

Write-Host "========================================" -ForegroundColor Green
Write-Host "? Deploy concluído!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "?? URL: https://$webAppUrl" -ForegroundColor Cyan
Write-Host "?? Health: https://$webAppUrl/api/health" -ForegroundColor Cyan
Write-Host ""
Write-Host "?? Logs:" -ForegroundColor Yellow
Write-Host "   az webapp log tail -n $webAppName -g $ResourceGroup" -ForegroundColor Gray
Write-Host ""
