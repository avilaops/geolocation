@description('Azure region for all resources')
param location string = resourceGroup().location

@description('Three-letter prefix used for resource naming (alphanumeric only).')
@minLength(2)
@maxLength(3)
param resourcePrefix string = 'geo'

@description('Five-character token appended to every resource name to ensure uniqueness.')
@minLength(5)
@maxLength(5)
param resourceToken string

@description('Docker image repository name stored in Azure Container Registry.')
param containerImageRepository string = 'geolocation'

@description('Docker image tag to deploy from Azure Container Registry.')
param containerImageTag string = 'latest'

@description('Container port exposed by the application image.')
param containerPort int = 8080

@description('Container Registry SKU. Basic is sufficient for CI/CD scenarios.')
@allowed([
  'Basic'
  'Standard'
  'Premium'
])
param acrSku string = 'Basic'

@description('App Service plan SKU (Linux). Ensure the selected tier supports your scale requirements.')
param appServiceSkuName string = 'S1'

@description('App Service plan tier (Linux).')
param appServiceSkuTier string = 'Standard'

@description('Provision Azure Key Vault for secret management.')
param enableKeyVault bool = true



@description('MongoDB Atlas connection string (required).')
@secure()
param mongodbUri string

@description('Google Maps API key (required).')
@secure()
param googleMapsApiKey string

var nameSeed = toLower('${resourcePrefix}${resourceToken}')
var acrName = replace(toLower('acr${nameSeed}'), '-', '')
var appServicePlanName = toLower('asp-${resourcePrefix}-${resourceToken}')
var webAppName = toLower('app-${resourcePrefix}-${resourceToken}')
var logAnalyticsName = toLower('law-${resourcePrefix}-${resourceToken}')
var appInsightsName = toLower('ai-${resourcePrefix}-${resourceToken}')
var keyVaultName = replace(toLower('kv-${resourcePrefix}-${resourceToken}'), '-', '')

var keyVaultMongoSecretName = 'mongodb-uri'
var keyVaultMapsSecretName = 'google-maps-api-key'
var keyVaultDnsSuffix = environment().suffixes.keyvaultDns
var mongoSecretUri = enableKeyVault ? format('https://{0}.{1}/secrets/{2}', keyVaultName, keyVaultDnsSuffix, keyVaultMongoSecretName) : ''
var mapsSecretUri = enableKeyVault ? format('https://{0}.{1}/secrets/{2}', keyVaultName, keyVaultDnsSuffix, keyVaultMapsSecretName) : ''

var commonTags = {
  Project: 'geolocation'
  Environment: 'dev'
  Owner: 'avilaops'
  ManagedBy: 'bicep'
}

resource acr 'Microsoft.ContainerRegistry/registries@2023-08-01-preview' = {
  name: acrName
  location: location
  tags: commonTags
  sku: {
    name: acrSku
  }
  properties: {
    adminUserEnabled: false
    publicNetworkAccess: 'Enabled'
    policies: {
      retentionPolicy: {
        status: 'Enabled'
        days: 14
      }
    }
  }
}

var dockerImageFullName = format('{0}/{1}:{2}', acr.properties.loginServer, containerImageRepository, containerImageTag)

resource logAnalytics 'Microsoft.OperationalInsights/workspaces@2022-10-01' = {
  name: logAnalyticsName
  location: location
  tags: commonTags
  properties: {
    sku: {
      name: 'PerGB2018'
    }
    retentionInDays: 30
    features: {
      enableLogAccessUsingOnlyResourcePermissions: true
    }
  }
}

resource appInsights 'Microsoft.Insights/components@2020-02-02' = {
  name: appInsightsName
  location: location
  tags: commonTags
  kind: 'web'
  properties: {
    Application_Type: 'web'
    Flow_Type: 'Bluefield'
    Request_Source: 'rest'
    WorkspaceResourceId: logAnalytics.id
  }
}

resource appServicePlan 'Microsoft.Web/serverfarms@2023-12-01' = {
  name: appServicePlanName
  location: location
  tags: commonTags
  kind: 'linux'
  sku: {
    name: appServiceSkuName
    tier: appServiceSkuTier
    size: appServiceSkuName
    capacity: 1
  }
  properties: {
    reserved: true
    zoneRedundant: false
    maximumElasticWorkerCount: 1
  }
}

var baseAppSettings = [
  {
    name: 'WEBSITES_PORT'
    value: string(containerPort)
  }
  {
    name: 'WEBSITES_ENABLE_APP_SERVICE_STORAGE'
    value: 'false'
  }
  {
    name: 'DOCKER_ENABLE_CI'
    value: 'true'
  }
  {
    name: 'APPLICATIONINSIGHTS_CONNECTION_STRING'
    value: appInsights.properties.ConnectionString
  }
  {
    name: 'RUST_LOG'
    value: 'info'
  }
]

var secretsAppSettings = enableKeyVault
  ? [
      {
        name: 'AZURE_KEY_VAULT_NAME'
        value: keyVaultName
      }
      {
        name: 'MONGODB_URI'
        value: format('@Microsoft.KeyVault(SecretUri={0})', mongoSecretUri)
      }
      {
        name: 'GOOGLE_MAPS_API_KEY'
        value: format('@Microsoft.KeyVault(SecretUri={0})', mapsSecretUri)
      }
    ]
  : [
      {
        name: 'MONGODB_URI'
        value: mongodbUri
      }
      {
        name: 'GOOGLE_MAPS_API_KEY'
        value: googleMapsApiKey
      }
    ]

resource webApp 'Microsoft.Web/sites@2023-12-01' = {
  name: webAppName
  location: location
  tags: commonTags
  kind: 'app,linux,container'
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    httpsOnly: true
    serverFarmId: appServicePlan.id
    siteConfig: {
      linuxFxVersion: format('DOCKER|{0}', dockerImageFullName)
      appSettings: concat(baseAppSettings, secretsAppSettings)
      alwaysOn: true
      ftpsState: 'Disabled'
      vnetRouteAllEnabled: true
      http20Enabled: true
      minTlsVersion: '1.2'
      healthCheckPath: '/api/health'
      acrUseManagedIdentityCreds: true
    }
  }
}

resource webAppLogs 'Microsoft.Insights/diagnosticSettings@2021-05-01-preview' = {
  name: '${webApp.name}-diagnostics'
  scope: webApp
  properties: {
    workspaceId: logAnalytics.id
    logs: [
      {
        category: 'AppServiceHTTPLogs'
        enabled: true
      }
      {
        category: 'AppServiceConsoleLogs'
        enabled: true
      }
      {
        category: 'AppServiceAppLogs'
        enabled: true
      }
    ]
    metrics: [
      {
        category: 'AllMetrics'
        enabled: true
      }
    ]
  }
}

// Commented out due to permission requirements
// resource acrPullAssignment 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
//   name: guid(webApp.id, 'acrpull')
//   scope: acr
//   properties: {
//     principalId: webApp.identity.principalId
//     roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', '7f951dda-4ed3-4680-a7ca-43fe172d538d')
//     principalType: 'ServicePrincipal'
//   }
// }

resource keyVault 'Microsoft.KeyVault/vaults@2023-07-01' = if (enableKeyVault) {
  name: keyVaultName
  location: location
  tags: commonTags
  properties: {
    tenantId: subscription().tenantId
    sku: {
      name: 'standard'
      family: 'A'
    }
    enableRbacAuthorization: true
    enabledForTemplateDeployment: true
    enabledForDiskEncryption: false
    enabledForDeployment: false
    softDeleteRetentionInDays: 90
    publicNetworkAccess: 'Enabled'
    networkAcls: {
      defaultAction: 'Allow'
      bypass: 'AzureServices'
    }
  }
}

resource keyVaultSecretsUser 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (enableKeyVault) {
  name: guid(keyVault.id, 'kv-secrets-user')
  scope: keyVault
  properties: {
    principalId: webApp.identity.principalId
    roleDefinitionId: subscriptionResourceId('Microsoft.Authorization/roleDefinitions', '4633458b-17de-408a-b874-0445c86b69e6')
    principalType: 'ServicePrincipal'
  }
}

resource keyVaultMongoSecret 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = if (enableKeyVault) {
  parent: keyVault
  name: keyVaultMongoSecretName
  properties: {
    value: mongodbUri
  }
}

resource keyVaultMapsSecret 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = if (enableKeyVault) {
  parent: keyVault
  name: keyVaultMapsSecretName
  properties: {
    value: googleMapsApiKey
  }
}
output containerRegistryLoginServer string = acr.properties.loginServer
output appServiceName string = webApp.name
output appServicePlan string = appServicePlan.name
output logAnalyticsWorkspace string = logAnalytics.name
output applicationInsightsName string = appInsights.name
output keyVaultProvisioned bool = enableKeyVault
output keyVaultName string = enableKeyVault ? keyVault.name : ''
