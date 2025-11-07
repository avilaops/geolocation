import { LucideIcon, TrendingUp, TrendingDown, Sparkles } from 'lucide-react'
import { useState } from 'react'

interface StatCardProps {
    title: string
    value: string | number
    icon: LucideIcon
    trend?: {
        value: number
        isPositive: boolean
    }
    color?: 'blue' | 'green' | 'purple' | 'orange'
    delay?: number
}

const colorClasses = {
    blue: {
        gradient: 'from-blue-500 to-indigo-600',
        glow: 'group-hover:shadow-blue-500/30',
        light: 'bg-blue-50',
        text: 'text-blue-600',
    },
    green: {
        gradient: 'from-emerald-500 to-teal-600',
        glow: 'group-hover:shadow-emerald-500/30',
        light: 'bg-emerald-50',
        text: 'text-emerald-600',
    },
    purple: {
        gradient: 'from-purple-500 to-pink-600',
        glow: 'group-hover:shadow-purple-500/30',
        light: 'bg-purple-50',
        text: 'text-purple-600',
    },
    orange: {
        gradient: 'from-orange-500 to-red-600',
        glow: 'group-hover:shadow-orange-500/30',
        light: 'bg-orange-50',
        text: 'text-orange-600',
    },
}

export default function StatCard({
    title,
    value,
    icon: Icon,
    trend,
    color = 'blue',
    delay = 0
}: StatCardProps) {
    const [isHovered, setIsHovered] = useState(false)
    const colorScheme = colorClasses[color]
    const TrendIcon = trend?.isPositive ? TrendingUp : TrendingDown

    return (
        <div
            className={`group relative bg-white rounded-2xl p-6 shadow-md border border-gray-100/50
                       hover:shadow-2xl ${colorScheme.glow} transition-all duration-500 
                       hover:-translate-y-2 cursor-pointer overflow-hidden animate-slide-up`}
            style={{ animationDelay: `${delay}ms` }}
            onMouseEnter={() => setIsHovered(true)}
            onMouseLeave={() => setIsHovered(false)}
        >
            {/* Efeito de brilho sutil no fundo */}
            <div className={`absolute inset-0 bg-gradient-to-br ${colorScheme.gradient} 
                           opacity-0 group-hover:opacity-5 transition-opacity duration-500`} />

            {/* Partícula flutuante */}
            <div className="absolute top-4 right-4 opacity-0 group-hover:opacity-20 transition-opacity duration-700">
                <Sparkles className={`w-4 h-4 ${colorScheme.text} animate-pulse`} />
            </div>

            <div className="relative flex items-start justify-between">
                <div className="flex-1">
                    <p className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-3">
                        {title}
                    </p>
                    <p className={`text-4xl font-extrabold text-gray-900 mb-2 transition-transform duration-300
                                  ${isHovered ? 'scale-105' : 'scale-100'}`}>
                        {typeof value === 'number' ? value.toLocaleString('pt-BR') : value}
                    </p>

                    {trend && (
                        <div className={`inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-sm font-bold
                                       ${trend.isPositive
                                ? 'bg-emerald-50 text-emerald-700 border border-emerald-200'
                                : 'bg-red-50 text-red-700 border border-red-200'}`}>
                            <TrendIcon className="w-4 h-4" />
                            <span>{Math.abs(trend.value)}%</span>
                        </div>
                    )}
                </div>

                <div className={`relative w-16 h-16 bg-gradient-to-br ${colorScheme.gradient} 
                               rounded-2xl flex items-center justify-center shadow-lg
                               group-hover:scale-110 group-hover:rotate-12 
                               transition-all duration-500`}>
                    <Icon className="w-8 h-8 text-white drop-shadow-lg" />

                    {/* Pulso animado no ícone */}
                    <div className={`absolute inset-0 bg-gradient-to-br ${colorScheme.gradient} 
                                   rounded-2xl animate-ping opacity-20`}
                        style={{ animationDuration: '3s' }} />
                </div>
            </div>

            {/* Barra de progresso inferior */}
            <div className="absolute bottom-0 left-0 right-0 h-1 bg-gray-50">
                <div className={`h-full bg-gradient-to-r ${colorScheme.gradient} 
                               transform origin-left transition-transform duration-700 ease-out
                               ${isHovered ? 'scale-x-100' : 'scale-x-0'}`} />
            </div>
        </div>
    )
}
