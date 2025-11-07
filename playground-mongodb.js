// Exemplo de uso da API MongoDB do Geolocation
// Use este arquivo no MongoDB Playground (VS Code extension) ou MongoDB Compass

// ============================================================
// 1. Conectar ao cluster (já configurado via connection string)
// ============================================================
use('geolocation');

// ============================================================
// 2. Consultar histórico de pesquisas recentes
// ============================================================
db.searches.find().sort({ timestamp: -1 }).limit(10);

// ============================================================
// 3. Estatísticas: Pesquisas por tipo
// ============================================================
db.searches.aggregate([
    {
        $group: {
            _id: "$search_type",
            count: { $sum: 1 },
            avg_duration_ms: { $avg: "$duration_ms" }
        }
    },
    { $sort: { count: -1 } }
]);

// ============================================================
// 4. Verificar cache de geocoding mais acessado
// ============================================================
db.geocoding_cache.find().sort({ access_count: -1 }).limit(5);

// ============================================================
// 5. Verificar cache de distância mais utilizado
// ============================================================
db.distance_matrix_cache.find().sort({ access_count: -1 }).limit(5);

// ============================================================
// 6. Limpar cache antigo (>30 dias sem acesso)
// ============================================================
const thirtyDaysAgo = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000);

db.geocoding_cache.deleteMany({
    last_accessed: { $lt: thirtyDaysAgo }
});

db.distance_matrix_cache.deleteMany({
    last_accessed: { $lt: thirtyDaysAgo }
});

// ============================================================
// 7. Criar índices (já criados automaticamente pelo servidor)
// ============================================================
db.searches.createIndex({ search_type: 1, timestamp: -1 });
db.searches.createIndex({ user_id: 1, timestamp: -1 });
db.geocoding_cache.createIndex({ normalized_address: 1 }, { unique: true });
db.geocoding_cache.createIndex({ last_accessed: -1 });
db.distance_matrix_cache.createIndex({ origin: 1, destination: 1, travel_mode: 1 }, { unique: true });
db.distance_matrix_cache.createIndex({ last_accessed: -1 });

// ============================================================
// 8. Estatísticas gerais do banco
// ============================================================
print("=== Estatísticas do Banco Geolocation ===");
print("Total de pesquisas:", db.searches.countDocuments());
print("Geocoding cache:", db.geocoding_cache.countDocuments());
print("Distance cache:", db.distance_matrix_cache.countDocuments());

// ============================================================
// 9. Top 10 endereços mais geocodificados
// ============================================================
db.geocoding_cache.aggregate([
    {
        $project: {
            address: 1,
            access_count: 1,
            formatted_address: 1
        }
    },
    { $sort: { access_count: -1 } },
    { $limit: 10 }
]);

// ============================================================
// 10. Pesquisas com erro
// ============================================================
db.searches.find({ error: { $exists: true, $ne: null } }).limit(10);
