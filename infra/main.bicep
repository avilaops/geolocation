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

@description('Provision Azure Database for PostgreSQL Flexible Server for production database workloads.')
param enablePostgres bool = false

@description('PostgreSQL administrator login name (required when enablePostgres = true).')
param postgresAdminUser string = 'pgadmin'

@description('PostgreSQL administrator password (override before deploying to production).')
@secure()
param postgresAdminPassword string = ''

@description('PostgreSQL version to deploy when enablePostgres = true.')
@allowed([
  '15'
  '16'
])
param postgresVersion string = '16'

@description('Compute SKU for the PostgreSQL Flexible Server.')
@allowed([
  'Standard_B1ms'
  'Standard_B2s'
  'Standard_D2s_v3'
])
param postgresSkuName string = 'Standard_B1ms'

@description('Storage size (GB) for PostgreSQL Flexible Server.')
@minValue(32)
@maxValue(1024)
param postgresStorageSizeGB int = 64

@description('Optional connection string that will be surfaced as an application setting when Key Vault is disabled.')
@secure()
param databaseConnectionString string = ''

@description('Optional Google Maps API key surfaced as an application setting when Key Vault is disabled.')
@secure()
param googleMapsApiKey string = ''

var nameSeed = toLower('${resourcePrefix}${resourceToken}')
var acrName = toLower('${nameSeed}acr01')
var appServicePlanName = toLower('${nameSeed}asp01')
var webAppName = toLower('${nameSeed}app01')
var logAnalyticsName = toLower('${nameSeed}law01')
var appInsightsName = toLower('${nameSeed}ai01')
var keyVaultName = toLower('${nameSeed}kv01')
var postgresServerName = toLower('${nameSeed}pg01')
var keyVaultDatabaseSecretName = 'database-url'
var keyVaultMapsSecretName = 'google-maps-api-key'
var keyVaultDnsSuffix = environment().suffixes.keyvaultDns
var databaseSecretUri = (enableKeyVault
  ? 'https://${keyVaultName}.${keyVaultDnsSuffix}/secrets/${keyVaultDatabaseSecretName}'
  : '')
var mapsSecretUri = (enableKeyVault
  ? 'https://${keyVaultName}.${keyVaultDnsSuffix}/secrets/${keyVaultMapsSecretName}'
  : '')
var secretsAppSettings = (enableKeyVault
  ? concat(
      [
        {
          name: 'AZURE_KEY_VAULT_NAME'
          value: keyVaultName
        }
      ],
      [
        {
          name: 'DATABASE_URL'
          value: '@Microsoft.KeyVault(SecretUri=${databaseSecretUri})'
        }
      ],
      [
        {
          name: 'GOOGLE_MAPS_API_KEY'
          value: '@Microsoft.KeyVault(SecretUri=${mapsSecretUri})'
        }
      ]
    )
  : concat(
      ((length(trim(databaseConnectionString)) > 0)
        ? [
            {
              name: 'DATABASE_URL'
              value: databaseConnectionString
            }
          ]
        : []),
      ((length(trim(googleMapsApiKey)) > 0)
        ? [
            {
              name: 'GOOGLE_MAPS_API_KEY'
              value: googleMapsApiKey
            }
          ]
        : [])
    ))

resource acr 'Microsoft.ContainerRegistry/registries@2023-08-01-preview' = {
  name: acrName
  location: location
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

resource logAnalytics 'Microsoft.OperationalInsights/workspaces@2022-10-01' = {
  name: logAnalyticsName
  location: location
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

resource webApp 'Microsoft.Web/sites@2023-12-01' = {
  name: webAppName
  location: location
  kind: 'app,linux,container'
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    httpsOnly: true
    serverFarmId: appServicePlan.id
    siteConfig: {
      linuxFxVersion: 'DOCKER|${acr.properties.loginServer}/${containerImageRepository}:${containerImageTag}'
      appSettings: concat(
        [
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
        ],
        secretsAppSettings
      )
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

resource webAppName_diagnostics 'Microsoft.Insights/diagnosticSettings@2021-05-01-preview' = {
  scope: webApp
  name: '${webAppName}-diagnostics'
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

resource Microsoft_Web_sites_webAppName_acrpull 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  scope: acr
  name: guid(webApp.id, 'acrpull')
  properties: {
    principalId: webApp.identity.principalId
    roleDefinitionId: subscriptionResourceId(
      'Microsoft.Authorization/roleDefinitions',
      '7f951dda-4ed3-4680-a7ca-43fe172d538d'
    )
    principalType: 'ServicePrincipal'
  }
}

resource keyVault 'Microsoft.KeyVault/vaults@2023-07-01' = if (enableKeyVault) {
  name: keyVaultName
  location: location
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

resource Microsoft_KeyVault_vaults_keyVaultName_kv_secrets_user 'Microsoft.Authorization/roleAssignments@2022-04-01' = if (enableKeyVault) {
  scope: keyVault
  name: guid(keyVault.id, 'kv-secrets-user')
  properties: {
  principalId: webApp.identity.principalId
    roleDefinitionId: subscriptionResourceId(
      'Microsoft.Authorization/roleDefinitions',
      '4633458b-17de-408a-b874-0445c86b69e6'
    )
    principalType: 'ServicePrincipal'
  }
}

resource keyVaultName_keyVaultDatabaseSecret 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = if (enableKeyVault && (length(trim(databaseConnectionString)) > 0)) {
  parent: keyVault
  name: keyVaultDatabaseSecretName
  properties: {
    value: databaseConnectionString
  }
}

resource keyVaultName_keyVaultMapsSecret 'Microsoft.KeyVault/vaults/secrets@2023-07-01' = if (enableKeyVault && (length(trim(googleMapsApiKey)) > 0)) {
  parent: keyVault
  name: keyVaultMapsSecretName
  properties: {
    value: googleMapsApiKey
  }
}

resource postgresServer 'Microsoft.DBforPostgreSQL/flexibleServers@2023-03-01-preview' = if (enablePostgres) {
  name: postgresServerName
  location: location
  properties: {
    version: postgresVersion
    administratorLogin: postgresAdminUser
    administratorLoginPassword: postgresAdminPassword
    storage: {
      storageSizeGB: postgresStorageSizeGB
    }
    backup: {
      backupRetentionDays: 7
      geoRedundantBackup: 'Disabled'
    }
  }
  sku: {
    name: postgresSkuName
    tier: 'Burstable'
  }
}

resource postgresServerName_geolocation 'Microsoft.DBforPostgreSQL/flexibleServers/databases@2023-03-01-preview' = if (enablePostgres) {
  parent: postgresServer
  name: 'geolocation'
  properties: {}
}

output containerRegistryLoginServer string = acr.properties.loginServer
output appServiceName string = webAppName
output appServicePlan string = appServicePlanName
output logAnalyticsWorkspace string = logAnalyticsName
output applicationInsightsName string = appInsightsName
output keyVaultProvisioned bool = enableKeyVault
output postgresProvisioned bool = enablePostgres
