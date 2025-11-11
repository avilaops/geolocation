# Guia de Deploy na Azure - Geolocation Project

## ?? Pré-requisitos

1. **Conta Azure** com permissões de Owner ou Contributor
2. **Azure CLI** instalado localmente
3. **GitHub** com acesso ao repositório
4. **Rust** e **Cargo** instalados (para build local)
5. **Node.js 20+** e **npm** (para frontend)
6. **Docker** instalado (para build de imagens)

---

## ?? Passo 1: Configurar Autenticação Azure com OIDC (GitHub Actions)

### 1.1 Criar um App Registration no Azure AD

```bash
# Login no Azure
az login

# Definir variáveis
SUBSCRIPTION_ID=$(az account show --query id -o tsv)
RESOURCE_GROUP="rg-geolocation"
APP_NAME="github-actions-geolocation"

# Criar App Registration
APP_ID=$(az ad app create --display-name $APP_NAME --query appId -o tsv)
echo "Application (Client) ID: $APP_ID"

# Criar Service Principal
SP_ID=$(az ad sp create --id $APP_ID --query id -o tsv)
echo "Service Principal ID: $SP_ID"

# Obter Tenant ID
TENANT_ID=$(az account show --query tenantId -o tsv)
echo "Tenant ID: $TENANT_ID"
```

### 1.2 Configurar Federated Credentials para OIDC

```bash
# Substitua pelos valores do seu repositório
GITHUB_ORG="avilaops"
GITHUB_REPO="geolocation"

# Criar credencial federada para branch main
az ad app federated-credential create \
  --id $APP_ID \
  --parameters "{
    \"name\": \"github-main-branch\",
  \"issuer\": \"https://token.actions.githubusercontent.com\",
    \"subject\": \"repo:${GITHUB_ORG}/${GITHUB_REPO}:ref:refs/heads/main\",
    \"audiences\": [\"api://AzureADTokenExchange\"]
  }"

# Criar credencial federada para ambiente 'dev' (opcional)
az ad app federated-credential create \
  --id $APP_ID \
  --parameters "{
    \"name\": \"github-env-dev\",
    \"issuer\": \"https://token.actions.githubusercontent.com\",
    \"subject\": \"repo:${GITHUB_ORG}/${GITHUB_REPO}:environment:dev\",
 \"audiences\": [\"api://AzureADTokenExchange\"]
  }"
```

### 1.3 Atribuir Permissões no Subscription

```bash
# Dar permissão de Contributor na subscription
az role assignment create \
  --assignee $APP_ID \
  --role Contributor \
  --scope /subscriptions/$SUBSCRIPTION_ID

echo "? Configuração OIDC concluída!"
echo ""
echo "?? Anote estes valores para configurar no GitHub:"
echo "AZURE_CLIENT_ID: $APP_ID"
echo "AZURE_TENANT_ID: $TENANT_ID"
echo "AZURE_SUBSCRIPTION_ID: $SUBSCRIPTION_ID"
```

---

## ?? Passo 2: Configurar Secrets no GitHub

### 2.1 Adicionar Secrets no Repositório

Vá para: `https://github.com/avilaops/geolocation/settings/secrets/actions`

Adicione os seguintes **Repository secrets**:

| Nome | Valor | Descrição |
|------|-------|-----------|
| `AZURE_CLIENT_ID` | (valor do $APP_ID) | Application (Client) ID |
| `AZURE_TENANT_ID` | (valor do $TENANT_ID) | Tenant ID |
| `AZURE_SUBSCRIPTION_ID` | (valor do $SUBSCRIPTION_ID) | Subscription ID |

### 2.2 Configurar Variables (Opcionais)

Vá para: `https://github.com/avilaops/geolocation/settings/variables/actions`

| Nome | Valor | Descrição |
|------|-------|-----------|
| `AZURE_RESOURCE_TOKEN` | `dev01` | Token único para recursos |

---

## ?? Passo 3: Configurar Parâmetros de Deploy

Já criamos arquivos de parâmetros para diferentes ambientes:

- `infra/parameters.dev.json` - Ambiente de desenvolvimento (Free tier)
- `infra/parameters.prod.json` - Produção (será criado a seguir)

---

## ?? Passo 4: Executar Deploy Manual (Primeira Vez)

### 4.1 Deploy via Azure CLI

```bash
# Login
az login

# Criar resource group
az group create \
  --name rg-geolocation \
  --location eastus

# Deploy da infraestrutura
az deployment group create \
  --resource-group rg-geolocation \
  --template-file infra/main.bicep \
  --parameters @infra/parameters.dev.json \
  --parameters resourceToken=dev01

# Capturar outputs
ACR_LOGIN_SERVER=$(az deployment group show \
  --resource-group rg-geolocation \
  --name <deployment-name> \
  --query properties.outputs.containerRegistryLoginServer.value -o tsv)

WEBAPP_NAME=$(az deployment group show \
  --resource-group rg-geolocation \
  --name <deployment-name> \
  --query properties.outputs.appServiceName.value -o tsv)

echo "ACR: $ACR_LOGIN_SERVER"
echo "Web App: $WEBAPP_NAME"
```

### 4.2 Build e Push da Imagem Docker

```bash
# Build da imagem
docker build -t geolocation:latest .

# Login no ACR
az acr login --name ${ACR_LOGIN_SERVER%%.*}

# Tag e push
docker tag geolocation:latest $ACR_LOGIN_SERVER/geolocation:latest
docker push $ACR_LOGIN_SERVER/geolocation:latest

# Configurar Web App para usar a imagem
az webapp config container set \
  --name $WEBAPP_NAME \
  --resource-group rg-geolocation \
  --docker-custom-image-name $ACR_LOGIN_SERVER/geolocation:latest \
  --docker-registry-server-url https://$ACR_LOGIN_SERVER

# Restart
az webapp restart --name $WEBAPP_NAME --resource-group rg-geolocation
```

---

## ?? Passo 5: Deploy Automatizado via GitHub Actions

Após configurar os secrets, cada push para `main` automaticamente:

1. ? Executa testes do backend (Rust)
2. ? Builda o frontend (React + Vite)
3. ? Cria imagem Docker
4. ? Faz deploy da infraestrutura (Bicep)
5. ? Publica imagem no ACR
6. ? Atualiza Web App com nova imagem

**Para disparar manualmente:**
1. Vá em: `https://github.com/avilaops/geolocation/actions`
2. Selecione workflow "Deploy to Azure"
3. Clique em "Run workflow"

---

## ?? Passo 6: Verificar Deploy

### 6.1 Verificar Web App

```bash
# Obter URL do Web App
WEBAPP_URL=$(az webapp show \
  --name $WEBAPP_NAME \
  --resource-group rg-geolocation \
  --query defaultHostName -o tsv)

echo "?? Aplicação disponível em: https://$WEBAPP_URL"
echo "?? Health check: https://$WEBAPP_URL/api/health"
```

### 6.2 Verificar Logs

```bash
# Ver logs em tempo real
az webapp log tail \
  --name $WEBAPP_NAME \
  --resource-group rg-geolocation

# Ou acessar via portal:
# https://portal.azure.com/#@/resource/subscriptions/{sub-id}/resourceGroups/rg-geolocation/providers/Microsoft.Web/sites/{webapp-name}/logStream
```

### 6.3 Verificar Application Insights

```bash
# Obter chave do App Insights
APPINSIGHTS_KEY=$(az monitor app-insights component show \
  --app geodev01ai01 \
  --resource-group rg-geolocation \
  --query instrumentationKey -o tsv)

echo "App Insights Key: $APPINSIGHTS_KEY"
```

---

## ?? Recursos Criados

Após o deploy, os seguintes recursos estarão disponíveis:

| Recurso | Nome | Tipo | Descrição |
|---------|------|------|-----------|
| Container Registry | `geodev01acr01` | ACR | Armazena imagens Docker |
| App Service Plan | `geodev01asp01` | Linux (Free/S1) | Plano de hospedagem |
| Web App | `geodev01app01` | Container | Aplicação principal |
| Log Analytics | `geodev01law01` | Workspace | Logs centralizados |
| Application Insights | `geodev01ai01` | APM | Monitoring e telemetria |
| Key Vault | `geodev01kv01` | (opcional) | Secrets management |
| PostgreSQL | `geodev01pg01` | (opcional) | Banco de dados |

---

## ??? Troubleshooting

### Problema: Container não inicia

```bash
# Ver logs detalhados
az webapp log download \
  --name $WEBAPP_NAME \
  --resource-group rg-geolocation \
  --log-file webapp-logs.zip

# Verificar configuração do container
az webapp config show \
  --name $WEBAPP_NAME \
  --resource-group rg-geolocation
```

### Problema: Erro de autenticação no ACR

```bash
# Verificar managed identity
az webapp identity show \
  --name $WEBAPP_NAME \
  --resource-group rg-geolocation

# Verificar role assignment
az role assignment list \
  --scope /subscriptions/$SUBSCRIPTION_ID/resourceGroups/rg-geolocation/providers/Microsoft.ContainerRegistry/registries/${ACR_NAME}
```

### Problema: Build falha no GitHub Actions

1. Verificar se os secrets estão configurados corretamente
2. Verificar logs do workflow
3. Testar build localmente: `cargo test && npm run build --prefix frontend`

---

## ?? Segurança

### Secrets Recomendados (se usar Key Vault)

Se habilitar `enableKeyVault: true` em `parameters.prod.json`:

```bash
# Adicionar connection string do banco
az keyvault secret set \
  --vault-name geodev01kv01 \
  --name database-url \
  --value "postgresql://user:pass@server/db"

# Adicionar Google Maps API Key
az keyvault secret set \
  --vault-name geodev01kv01 \
  --name google-maps-api-key \
  --value "YOUR_API_KEY"
```

---

## ?? Monitoramento

### Application Insights Queries (KQL)

```kusto
// Requests HTTP nos últimos 30min
requests
| where timestamp > ago(30m)
| summarize count() by resultCode, bin(timestamp, 1m)

// Erros
exceptions
| where timestamp > ago(1h)
| project timestamp, type, outerMessage

// Performance
requests
| where timestamp > ago(1h)
| summarize avg(duration), percentile(duration, 95) by name
```

---

## ?? Próximos Passos

1. ? **Deploy Inicial** - Concluído seguindo este guia
2. ?? **CI/CD Automatizado** - GitHub Actions configurado
3. ?? **Configurar Key Vault** - Para secrets em produção
4. ??? **Configurar PostgreSQL** - Para persistência de dados
5. ?? **Custom Domain** - Adicionar domínio personalizado
6. ?? **SSL/TLS** - Configurar certificado (gerenciado automaticamente)
7. ?? **Dashboards** - Criar dashboards no Application Insights
8. ?? **Alertas** - Configurar alertas de saúde e performance

---

## ?? Referências

- [Azure App Service Docs](https://learn.microsoft.com/en-us/azure/app-service/)
- [Azure Container Registry](https://learn.microsoft.com/en-us/azure/container-registry/)
- [Bicep Documentation](https://learn.microsoft.com/en-us/azure/azure-resource-manager/bicep/)
- [GitHub Actions Azure Login](https://github.com/Azure/login)

---

**Criado em:** $(Get-Date -Format "yyyy-MM-dd HH:mm")
**Versão:** 1.0
