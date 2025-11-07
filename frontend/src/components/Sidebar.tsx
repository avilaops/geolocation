import { NavLink } from 'react-router-dom'
import {
    LayoutDashboard,
    Upload,
    FileText,
    Truck,
    BarChart3,
    Settings,
    Zap,
} from 'lucide-react'

const navigation = [
    { name: 'Dashboard', to: '/', icon: LayoutDashboard },
    { name: 'Upload', to: '/upload', icon: Upload },
    { name: 'Notas Fiscais', to: '/notas-fiscais', icon: FileText },
    { name: 'CT-e', to: '/ctes', icon: Truck },
    { name: 'Análise Fiscal', to: '/analise-fiscal', icon: BarChart3 },
    { name: 'Configurações', to: '/settings', icon: Settings },
]

export default function Sidebar() {
    return (
        <div className="w-64 bg-gradient-to-b from-primary-900 to-primary-800 text-white flex flex-col">
            {/* Logo Premium com animação */}
            <div className="group p-6 flex items-center gap-4 border-b border-primary-700 cursor-pointer
                          hover:bg-primary-700/30 transition-all duration-300">
                <div className="relative w-12 h-12 bg-gradient-to-br from-yellow-400 to-amber-500 
                              rounded-xl flex items-center justify-center shadow-lg
                              group-hover:shadow-2xl group-hover:scale-110 group-hover:rotate-12 
                              transition-all duration-500 animate-fade-in">
                    <Zap className="w-7 h-7 text-primary-900 group-hover:animate-pulse" />
                    <div className="absolute inset-0 rounded-xl bg-white opacity-0 group-hover:opacity-20 transition-opacity duration-300" />
                </div>
                <div className="flex-1">
                    <h1 className="text-xl font-bold group-hover:text-yellow-300 transition-colors duration-300">
                        Geolocation
                    </h1>
                    <p className="text-xs text-primary-300 font-medium group-hover:text-yellow-400 transition-colors duration-300">
                        ⚡ v0.1.0 Premium
                    </p>
                </div>
            </div>

            {/* Navigation com micro-interações premium */}
            <nav className="flex-1 p-4 space-y-2">
                {navigation.map((item, index) => (
                    <NavLink
                        key={item.name}
                        to={item.to}
                        className={({ isActive }) =>
                            `group relative flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-300 
                             hover:translate-x-2 hover:shadow-xl animate-slide-in
                             ${isActive
                                ? 'bg-white text-primary-900 shadow-2xl font-semibold'
                                : 'text-primary-100 hover:bg-primary-700/50 hover:text-white'
                            }`
                        }
                        style={{ animationDelay: `${index * 50}ms` }}
                    >
                        {/* Barra lateral de destaque no item ativo */}
                        {({ isActive }) => (
                            <>
                                {isActive && (
                                    <div className="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-8 bg-primary-500 rounded-r-full animate-scale-in" />
                                )}

                                {/* Ícone com animação */}
                                <item.icon className={`w-5 h-5 transition-transform duration-300 group-hover:scale-110 group-hover:rotate-12
                                    ${isActive ? 'drop-shadow-lg' : ''}`} />

                                {/* Texto com animação */}
                                <span className="font-medium transition-all duration-300 group-hover:font-semibold">
                                    {item.name}
                                </span>

                                {/* Brilho no hover */}
                                <div className="absolute inset-0 rounded-xl bg-gradient-to-r from-transparent via-white/10 to-transparent 
                                              opacity-0 group-hover:opacity-100 group-hover:translate-x-full transition-all duration-700 -z-10" />
                            </>
                        )}
                    </NavLink>
                ))}
            </nav>

            {/* Footer */}
            <div className="p-4 border-t border-primary-700">
                <div className="text-xs text-primary-300 text-center">
                    <p>Powered by Rust + Assembly</p>
                    <p className="mt-1">© 2025 Avila DevOps</p>
                </div>
            </div>
        </div>
    )
}
