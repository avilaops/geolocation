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

@description('MongoDB Atlas connection string (required).')
@secure()
param mongodbUri string

@description('Google Maps API key (required).')
@secure()
param googleMapsApiKey string

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

// Use existing Container Apps environment
resource existingManagedEnvironment 'Microsoft.App/managedEnvironments@2023-05-01' existing = {
  name: 'shancrys-env-dev-hp7owgdw2zpma'
}

// Container App
resource containerApp 'Microsoft.App/containerApps@2023-05-01' = {
  name: '${nameSeed}-app'
  location: location
  properties: {
    managedEnvironmentId: existingManagedEnvironment.id
    configuration: {
      activeRevisionsMode: 'Single'
      ingress: {
        external: true
        targetPort: containerPort
        allowInsecure: false
      }
      registries: [
        {
          server: acr.properties.loginServer
          username: acr.listCredentials().username
          passwordSecretRef: 'acr-password'
        }
      ]
      secrets: [
        {
          name: 'acr-password'
          value: acr.listCredentials().passwords[0].value
        }
        {
          name: 'mongodb-uri'
          value: mongodbUri
        }
        {
          name: 'google-maps-api-key'
          value: googleMapsApiKey
        }
      ]
    }
    template: {
      containers: [
        {
          image: '${acr.properties.loginServer}/${containerImageRepository}:${containerImageTag}'
          name: 'geolocation'
          env: [
            {
              name: 'MONGODB_URI'
              secretRef: 'mongodb-uri'
            }
            {
              name: 'GOOGLE_MAPS_API_KEY'
              secretRef: 'google-maps-api-key'
            }
            {
              name: 'PORT'
              value: string(containerPort)
            }
          ]
          resources: {
            cpu: json('0.25')
            memory: '0.5Gi'
          }
        }
      ]
      scale: {
        minReplicas: 1
        maxReplicas: 3
      }
    }
  }
}

output containerRegistryLoginServer string = acr.properties.loginServer
output containerAppUrl string = 'https://${containerApp.properties.configuration.ingress.fqdn}'
output containerAppName string = containerApp.name