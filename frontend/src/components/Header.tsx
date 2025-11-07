import { Bell, User, Search, Cpu, AlertTriangle } from 'lucide-react'
import { useStore } from '../store/useStore'
import { useWasm } from '../hooks/useWasm'

export default function Header() {
    const { stats } = useStore()
    const { wasm, loading } = useWasm()

    return (
        <header className="bg-white border-b border-gray-200 px-6 py-4 backdrop-blur-lg shadow-sm">
            <div className="flex items-center justify-between">
                {/* Search Bar Premium com animação de foco */}
                <div className="flex-1 max-w-lg">
                    <div className="relative group">
                        <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400 
                                         group-focus-within:text-indigo-600 transition-colors duration-300" />
                        <input
                            type="text"
                            placeholder="🔍 Buscar documentos por chave de acesso..."
                            className="w-full pl-10 pr-4 py-2.5 border-2 border-gray-300 rounded-xl
                                     focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500
                                     transition-all duration-300 hover:border-indigo-300
                                     focus:shadow-lg focus:shadow-indigo-100"
                        />
                        {/* Gradiente de foco animado */}
                        <div className="absolute inset-0 rounded-xl bg-gradient-to-r from-indigo-500 to-purple-500 
                                      opacity-0 group-focus-within:opacity-10 transition-opacity duration-300 -z-10 blur-xl" />
                    </div>
                </div>

                {/* Stats & Actions */}
                <div className="flex items-center gap-6">
                    {/* WASM Status Badge Premium com animação */}
                    {!loading && wasm && (
                        <div
                            className={`hidden md:flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-semibold
                                       shadow-md hover:shadow-lg transition-all duration-300 hover:scale-105 animate-fade-in
                                       ${(wasm as any).__isStub
                                    ? 'bg-gradient-to-r from-yellow-100 to-amber-100 text-yellow-800 border-2 border-yellow-400'
                                    : 'bg-gradient-to-r from-green-100 to-emerald-100 text-green-800 border-2 border-green-400'
                                }`}
                            title={
                                (wasm as any).__isStub
                                    ? 'Usando stub - Execute build-wasm.ps1 para módulo real'
                                    : 'Módulo WASM real carregado'
                            }
                        >
                            {(wasm as any).__isStub ? (
                                <>
                                    <AlertTriangle className="w-4 h-4 animate-pulse" />
                                    <span>WASM Stub</span>
                                </>
                            ) : (
                                <>
                                    <Cpu className="w-4 h-4 animate-pulse" />
                                    <span>✨ WASM Real</span>
                                </>
                            )}
                        </div>
                    )}

                    {/* Quick Stats Premium com hover effects */}
                    <div className="hidden md:flex items-center gap-4 text-sm">
                        <div className="group text-center px-5 py-2 border-r border-gray-200 hover:bg-gradient-to-br hover:from-indigo-50 hover:to-purple-50 
                                      rounded-lg transition-all duration-300 hover:scale-105 cursor-pointer">
                            <div className="text-2xl font-bold bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent
                                          group-hover:scale-110 transition-transform duration-300">
                                {stats.totalDocuments.toLocaleString('pt-BR')}
                            </div>
                            <div className="text-xs text-gray-500 font-medium">📄 Documentos</div>
                        </div>
                        <div className="group text-center px-5 py-2 hover:bg-gradient-to-br hover:from-green-50 hover:to-emerald-50 
                                      rounded-lg transition-all duration-300 hover:scale-105 cursor-pointer">
                            <div className="text-2xl font-bold bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent
                                          group-hover:scale-110 transition-transform duration-300">
                                {stats.processedToday.toLocaleString('pt-BR')}
                            </div>
                            <div className="text-xs text-gray-500 font-medium">🚀 Hoje</div>
                        </div>
                    </div>

                    {/* Notifications Premium com badge animado */}
                    <button className="relative p-3 text-gray-600 hover:bg-gradient-to-br hover:from-indigo-50 hover:to-purple-50 
                                     rounded-xl transition-all duration-300 hover:scale-110 group">
                        <Bell className="w-5 h-5 group-hover:text-indigo-600 transition-colors duration-300 group-hover:animate-pulse" />
                        <span className="absolute top-2 right-2 w-2.5 h-2.5 bg-red-500 rounded-full animate-ping"></span>
                        <span className="absolute top-2 right-2 w-2.5 h-2.5 bg-red-500 rounded-full"></span>
                    </button>

                    {/* User Menu Premium com shadow e animação */}
                    <button className="group flex items-center gap-3 px-3 py-2 hover:bg-gradient-to-br hover:from-gray-50 hover:to-indigo-50 
                                     rounded-xl transition-all duration-300 hover:shadow-lg hover:scale-105">
                        <div className="relative w-10 h-10 bg-gradient-to-br from-indigo-500 via-purple-600 to-pink-600 
                                      rounded-full flex items-center justify-center shadow-md
                                      group-hover:shadow-xl group-hover:scale-110 transition-all duration-300">
                            <User className="w-5 h-5 text-white" />
                            <div className="absolute inset-0 rounded-full bg-white opacity-0 group-hover:opacity-20 transition-opacity duration-300" />
                        </div>
                        <div className="hidden md:block text-left text-sm">
                            <div className="font-semibold text-gray-800 group-hover:text-indigo-700 transition-colors duration-300">
                                Admin
                            </div>
                            <div className="text-xs text-gray-500 font-medium">👑 Administrador</div>
                        </div>
                    </button>
                </div>
            </div>
        </header>
    )
}
