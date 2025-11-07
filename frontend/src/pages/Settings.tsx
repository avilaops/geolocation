import { Database, Server, HardDrive } from 'lucide-react'

export default function Settings() {
    return (
        <div className="space-y-6">
            {/* Título */}
            <div>
                <h1 className="text-3xl font-bold text-gray-900">Configurações</h1>
                <p className="text-gray-600 mt-1">
                    Configure o banco de dados e preferências do sistema
                </p>
            </div>

            {/* Configuração do Banco de Dados */}
            <div className="card">
                <div className="flex items-center gap-3 mb-6">
                    <div className="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
                        <Database className="w-6 h-6 text-blue-600" />
                    </div>
                    <div>
                        <h2 className="text-xl font-semibold text-gray-900">
                            Banco de Dados
                        </h2>
                        <p className="text-sm text-gray-600">
                            Configure a conexão com o banco de dados
                        </p>
                    </div>
                </div>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Tipo de Banco
                        </label>
                        <select className="input-field">
                            <option value="sqlite">SQLite (Padrão)</option>
                            <option value="postgresql">PostgreSQL</option>
                        </select>
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Caminho do Banco (SQLite)
                        </label>
                        <input
                            type="text"
                            className="input-field"
                            placeholder="./geolocation.db"
                            defaultValue="./geolocation.db"
                        />
                    </div>

                    <div className="pt-4 border-t border-gray-200">
                        <h3 className="text-sm font-medium text-gray-900 mb-3">
                            PostgreSQL (Opcional)
                        </h3>
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                            <div>
                                <label className="block text-sm text-gray-700 mb-1">Host</label>
                                <input
                                    type="text"
                                    className="input-field"
                                    placeholder="localhost"
                                />
                            </div>
                            <div>
                                <label className="block text-sm text-gray-700 mb-1">Porta</label>
                                <input
                                    type="text"
                                    className="input-field"
                                    placeholder="5432"
                                />
                            </div>
                            <div>
                                <label className="block text-sm text-gray-700 mb-1">Usuário</label>
                                <input
                                    type="text"
                                    className="input-field"
                                    placeholder="postgres"
                                />
                            </div>
                            <div>
                                <label className="block text-sm text-gray-700 mb-1">Senha</label>
                                <input
                                    type="password"
                                    className="input-field"
                                    placeholder="••••••"
                                />
                            </div>
                        </div>
                    </div>

                    <div className="flex justify-end gap-2 pt-4">
                        <button className="btn-secondary">Testar Conexão</button>
                        <button className="btn-primary">Salvar Configurações</button>
                    </div>
                </div>
            </div>

            {/* Configuração do Servidor */}
            <div className="card">
                <div className="flex items-center gap-3 mb-6">
                    <div className="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center">
                        <Server className="w-6 h-6 text-green-600" />
                    </div>
                    <div>
                        <h2 className="text-xl font-semibold text-gray-900">
                            Servidor Backend
                        </h2>
                        <p className="text-sm text-gray-600">
                            Configurações do servidor Rust
                        </p>
                    </div>
                </div>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Porta do Servidor
                        </label>
                        <input
                            type="text"
                            className="input-field"
                            placeholder="8080"
                            defaultValue="8080"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">
                            Diretório de Upload
                        </label>
                        <input
                            type="text"
                            className="input-field"
                            placeholder="./uploads"
                            defaultValue="./uploads"
                        />
                    </div>

                    <div className="flex items-center gap-2">
                        <input
                            type="checkbox"
                            id="auto-process"
                            className="w-4 h-4 text-primary-600 rounded"
                            defaultChecked
                        />
                        <label htmlFor="auto-process" className="text-sm text-gray-700">
                            Processar arquivos automaticamente após upload
                        </label>
                    </div>

                    <div className="flex justify-end pt-4">
                        <button className="btn-primary">Salvar Configurações</button>
                    </div>
                </div>
            </div>

            {/* Armazenamento */}
            <div className="card">
                <div className="flex items-center gap-3 mb-6">
                    <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
                        <HardDrive className="w-6 h-6 text-purple-600" />
                    </div>
                    <div>
                        <h2 className="text-xl font-semibold text-gray-900">
                            Armazenamento
                        </h2>
                        <p className="text-sm text-gray-600">
                            Informações sobre uso de disco
                        </p>
                    </div>
                </div>

                <div className="space-y-4">
                    <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                        <div>
                            <p className="text-sm font-medium text-gray-900">
                                Banco de Dados
                            </p>
                            <p className="text-xs text-gray-500">geolocation.db</p>
                        </div>
                        <p className="text-lg font-bold text-gray-900">12.5 MB</p>
                    </div>

                    <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
                        <div>
                            <p className="text-sm font-medium text-gray-900">
                                Arquivos XML
                            </p>
                            <p className="text-xs text-gray-500">uploads/</p>
                        </div>
                        <p className="text-lg font-bold text-gray-900">48.2 MB</p>
                    </div>

                    <div className="flex items-center justify-between p-4 bg-primary-50 rounded-lg border border-primary-200">
                        <div>
                            <p className="text-sm font-medium text-primary-900">
                                Total Utilizado
                            </p>
                            <p className="text-xs text-primary-600">Todos os arquivos</p>
                        </div>
                        <p className="text-lg font-bold text-primary-900">60.7 MB</p>
                    </div>

                    <div className="flex justify-end pt-4">
                        <button className="btn-secondary text-red-600 hover:bg-red-50">
                            Limpar Cache
                        </button>
                    </div>
                </div>
            </div>
        </div>
    )
}
