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

@description('Container Registry SKU. Basic is sufficient for CI/CD scenarios.')
@allowed([
  'Basic'
  'Standard'
  'Premium'
])
param acrSku string = 'Basic'

var nameSeed = toLower('${resourcePrefix}${resourceToken}')

// Container Registry
resource acr 'Microsoft.ContainerRegistry/registries@2023-07-01' = {
  name: 'acr${nameSeed}'
  location: location
  sku: {
    name: acrSku
  }
  properties: {
    adminUserEnabled: true
    dataEndpointEnabled: false
    encryption: {
      status: 'disabled'
    }
    networkRuleBypassOptions: 'AzureServices'
    publicNetworkAccess: 'Enabled'
    zoneRedundancy: 'Disabled'
  }
}

output containerRegistryLoginServer string = acr.properties.loginServer
output containerRegistryName string = acr.name