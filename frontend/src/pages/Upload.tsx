import { useState, useCallback } from 'react'
import { useDropzone } from 'react-dropzone'
import { Upload as UploadIcon, X, CheckCircle, AlertCircle, Loader, FileText, Shield, AlertTriangle, Info, XCircle } from 'lucide-react'
import toast from 'react-hot-toast'
import { documentService, ProcessResult, ValidationResult } from '../services/api'
// import { useWasm } from '../hooks/useWasm'

interface FileWithStatus {
    file: File
    status: 'pending' | 'uploading' | 'success' | 'error'
    result?: ProcessResult
    error?: string
}

export default function Upload() {
    const [files, setFiles] = useState<FileWithStatus[]>([])
    const [isProcessing, setIsProcessing] = useState(false)
    // const { parseDocument, loading: wasmLoading, error: wasmError } = useWasm()

    const onDrop = useCallback((acceptedFiles: File[]) => {
        const newFiles = acceptedFiles.map((file) => ({
            file,
            status: 'pending' as const,
        }))
        setFiles((prev) => [...prev, ...newFiles])

        toast.success(
            `🎉 ${acceptedFiles.length} arquivo${acceptedFiles.length > 1 ? 's' : ''} adicionado${acceptedFiles.length > 1 ? 's' : ''} com sucesso!`,
            {
                duration: 3000,
                icon: '📁',
                style: {
                    background: 'linear-gradient(135deg, #3b82f6 0%, #2563eb 100%)',
                    color: '#fff',
                    fontWeight: '600',
                    fontSize: '14px',
                    borderRadius: '16px',
                    boxShadow: '0 10px 30px rgba(59, 130, 246, 0.3)',
                    border: '2px solid #60a5fa',
                },
            }
        )
    }, [])

    const { getRootProps, getInputProps, isDragActive } = useDropzone({
        onDrop,
        accept: {
            'text/xml': ['.xml'],
            'application/xml': ['.xml'],
        },
        multiple: true,
        maxSize: 10485760, // 10MB
    })

    const removeFile = (index: number) => {
        setFiles((prev) => prev.filter((_, i) => i !== index))
    }

    const processFiles = async () => {
        if (files.length === 0) {
            toast.error(
                '⚠️ Adicione pelo menos um arquivo XML antes de processar!',
                {
                    duration: 4000,
                    icon: '📋',
                    style: {
                        background: 'linear-gradient(135deg, #f59e0b 0%, #d97706 100%)',
                        color: '#fff',
                        fontWeight: '600',
                        fontSize: '14px',
                        borderRadius: '16px',
                        boxShadow: '0 10px 30px rgba(245, 158, 11, 0.3)',
                        border: '2px solid #fbbf24',
                    },
                }
            )
            return
        }

        setIsProcessing(true)

        for (let i = 0; i < files.length; i++) {
            const fileWithStatus = files[i]

            if (fileWithStatus.status !== 'pending') continue

            // Atualizar status para "uploading"
            setFiles((prev) =>
                prev.map((f, idx) =>
                    idx === i ? { ...f, status: 'uploading' as const } : f
                )
            )

            try {
                const result = await documentService.uploadFile(fileWithStatus.file)

                // Atualizar status para "success"
                setFiles((prev) =>
                    prev.map((f, idx) =>
                        idx === i
                            ? { ...f, status: 'success' as const, result }
                            : f
                    )
                )

                if (result.duplicate) {
                    toast(
                        () => (
                            <div className="text-sm font-medium text-yellow-900">
                                ⚠️ Documento já existente<br />
                                <span className="font-mono text-xs">{result.chave_acesso}</span>
                            </div>
                        ),
                        {
                            duration: 5000,
                            style: {
                                background: 'linear-gradient(135deg,#fde047,#facc15)',
                                color: '#000',
                                borderRadius: '14px',
                                border: '2px solid #fbbf24',
                            },
                        }
                    )
                } else {
                    toast.success(
                        `✨ ${fileWithStatus.file.name} processado com sucesso!`,
                        {
                            duration: 4000,
                            icon: '✅',
                            style: {
                                background: 'linear-gradient(135deg, #10b981 0%, #059669 100%)',
                                color: '#fff',
                                fontWeight: '600',
                                fontSize: '14px',
                                borderRadius: '16px',
                                boxShadow: '0 10px 30px rgba(16, 185, 129, 0.3)',
                                border: '2px solid #34d399',
                            },
                        }
                    )
                }
            } catch (error: any) {
                // Atualizar status para "error"
                setFiles((prev) =>
                    prev.map((f, idx) =>
                        idx === i
                            ? {
                                ...f,
                                status: 'error' as const,
                                error: error.response?.data?.message || 'Erro ao processar arquivo',
                            }
                            : f
                    )
                )

                toast.error(
                    `⚠️ Erro ao processar ${fileWithStatus.file.name}`,
                    {
                        duration: 5000,
                        icon: '❌',
                        style: {
                            background: 'linear-gradient(135deg, #ef4444 0%, #dc2626 100%)',
                            color: '#fff',
                            fontWeight: '600',
                            fontSize: '14px',
                            borderRadius: '16px',
                            boxShadow: '0 10px 30px rgba(239, 68, 68, 0.3)',
                            border: '2px solid #f87171',
                        },
                    }
                )
            }
        }

        setIsProcessing(false)
        toast.success('Processamento concluído!')
    }

    const clearAll = () => {
        setFiles([])
    }

    const getStatusIcon = (status: FileWithStatus['status']) => {
        switch (status) {
            case 'uploading':
                return <Loader className="w-5 h-5 text-blue-500 animate-spin" />
            case 'success':
                return <CheckCircle className="w-5 h-5 text-green-500" />
            case 'error':
                return <AlertCircle className="w-5 h-5 text-red-500" />
            default:
                return null
        }
    }

    const renderValidationBadges = (validation: ValidationResult | undefined) => {
        if (!validation) return null

        const hasErrors = validation.errors.length > 0
        const hasWarnings = validation.warnings.length > 0

        if (!hasErrors && !hasWarnings) {
            return (
                <div className="flex items-center gap-2 mt-2 animate-fade-in">
                    <div className="flex items-center gap-1 px-3 py-1 bg-green-100 border border-green-300 rounded-lg">
                        <Shield className="w-4 h-4 text-green-700" />
                        <span className="text-xs font-bold text-green-800">✓ Validado</span>
                    </div>
                </div>
            )
        }

        return (
            <div className="mt-3 space-y-2 animate-fade-in">
                {/* Errors */}
                {validation.errors.map((error, idx) => (
                    <div key={idx} className="flex items-start gap-2 p-2 bg-red-50 border border-red-200 rounded-lg">
                        <XCircle className="w-4 h-4 text-red-600 flex-shrink-0 mt-0.5" />
                        <div className="flex-1 min-w-0">
                            <p className="text-xs font-bold text-red-800">{error.field}</p>
                            <p className="text-xs text-red-700">{error.message}</p>
                            <span className="inline-block mt-1 px-2 py-0.5 bg-red-200 text-red-900 text-[10px] font-bold rounded">
                                {error.severity}
                            </span>
                        </div>
                    </div>
                ))}

                {/* Warnings */}
                {validation.warnings.map((warning, idx) => (
                    <div key={idx} className="flex items-start gap-2 p-2 bg-yellow-50 border border-yellow-200 rounded-lg">
                        <AlertTriangle className="w-4 h-4 text-yellow-600 flex-shrink-0 mt-0.5" />
                        <div className="flex-1 min-w-0">
                            <p className="text-xs font-bold text-yellow-800">{warning.field}</p>
                            <p className="text-xs text-yellow-700">{warning.message}</p>
                            <p className="text-[10px] text-yellow-600 mt-1">💡 {warning.impact}</p>
                        </div>
                    </div>
                ))}

                {/* Suggestions */}
                {validation.suggestions.length > 0 && (
                    <div className="flex items-start gap-2 p-2 bg-blue-50 border border-blue-200 rounded-lg">
                        <Info className="w-4 h-4 text-blue-600 flex-shrink-0 mt-0.5" />
                        <div className="flex-1 min-w-0">
                            <p className="text-xs font-bold text-blue-800 mb-1">Sugestões:</p>
                            {validation.suggestions.map((suggestion, idx) => (
                                <p key={idx} className="text-xs text-blue-700">• {suggestion}</p>
                            ))}
                        </div>
                    </div>
                )}
            </div>
        )
    }

    return (
        <div className="space-y-6">
            {/* Título */}
            <div>
                <h1 className="text-3xl font-bold text-gray-900">Upload de Documentos</h1>
                <p className="text-gray-600 mt-1">
                    Faça upload de arquivos XML (NF-e ou CT-e) para processamento
                </p>
            </div>

            {/* Área de Drop Premium */}
            <div className="relative group">
                <div className="absolute inset-0 bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 rounded-3xl opacity-0 group-hover:opacity-20 blur-xl transition-opacity duration-500" />
                <div className="relative bg-white rounded-3xl shadow-xl overflow-hidden">
                    <div
                        {...getRootProps()}
                        className={`relative border-2 border-dashed rounded-3xl p-16 text-center cursor-pointer transition-all duration-500
                            ${isDragActive
                                ? 'border-indigo-500 bg-gradient-to-br from-indigo-50 to-purple-50 scale-[1.02]'
                                : 'border-gray-300 hover:border-indigo-400 hover:bg-gradient-to-br hover:from-gray-50 hover:to-indigo-50/30'
                            }`}
                    >
                        <input {...getInputProps()} />

                        {/* Ícone animado */}
                        <div className={`relative mx-auto w-24 h-24 mb-6 transition-transform duration-500
                                       ${isDragActive ? 'scale-110 rotate-12' : 'group-hover:scale-105'}`}>
                            <div className="absolute inset-0 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl opacity-10 animate-pulse" />
                            <div className="relative flex items-center justify-center w-full h-full">
                                <UploadIcon className={`w-14 h-14 transition-colors duration-300
                                    ${isDragActive ? 'text-indigo-600' : 'text-gray-400 group-hover:text-indigo-500'}`} />
                            </div>
                        </div>

                        {isDragActive ? (
                            <div className="animate-scale-in">
                                <p className="text-2xl text-indigo-600 font-bold mb-2">
                                    ✨ Solte os arquivos aqui!
                                </p>
                                <p className="text-sm text-indigo-500 font-medium">
                                    Processamento instantâneo iniciará automaticamente
                                </p>
                            </div>
                        ) : (
                            <>
                                <p className="text-2xl text-gray-800 font-bold mb-3">
                                    Arraste seus arquivos XML aqui
                                </p>
                                <p className="text-gray-500 mb-6 text-lg">
                                    ou clique para <span className="text-indigo-600 font-semibold">selecionar do computador</span>
                                </p>
                                <div className="flex items-center justify-center gap-6 text-sm text-gray-400">
                                    <span className="flex items-center gap-2">
                                        <CheckCircle className="w-4 h-4 text-green-500" />
                                        NF-e e CT-e suportados
                                    </span>
                                    <span className="flex items-center gap-2">
                                        <CheckCircle className="w-4 h-4 text-green-500" />
                                        Até 10MB por arquivo
                                    </span>
                                    <span className="flex items-center gap-2">
                                        <CheckCircle className="w-4 h-4 text-green-500" />
                                        Upload múltiplo
                                    </span>
                                </div>
                            </>
                        )}
                    </div>
                </div>
            </div>

            {/* Lista de Arquivos Premium */}
            {files.length > 0 && (
                <div className="relative animate-slide-up" style={{ animationDelay: '200ms' }}>
                    <div className="bg-white rounded-3xl shadow-xl overflow-hidden">
                        {/* Header com gradiente */}
                        <div className="relative bg-gradient-to-r from-indigo-600 via-purple-600 to-pink-600 px-8 py-6">
                            <div className="absolute inset-0 bg-black opacity-5" />
                            <div className="relative flex items-center justify-between">
                                <div className="flex items-center gap-4">
                                    <div className="bg-white/20 backdrop-blur-md rounded-xl px-4 py-2">
                                        <h3 className="text-xl font-bold text-white flex items-center gap-2">
                                            <FileText className="w-5 h-5" />
                                            {files.length} {files.length === 1 ? 'Arquivo' : 'Arquivos'}
                                        </h3>
                                    </div>
                                    {isProcessing && (
                                        <div className="flex items-center gap-2 text-white/90 text-sm font-medium">
                                            <Loader className="w-4 h-4 animate-spin" />
                                            <span>Processando...</span>
                                        </div>
                                    )}
                                </div>
                                <div className="flex gap-3">
                                    <button
                                        onClick={clearAll}
                                        className="px-4 py-2 bg-white/10 hover:bg-white/20 backdrop-blur-md text-white rounded-xl font-medium transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed"
                                        disabled={isProcessing}
                                    >
                                        Limpar Todos
                                    </button>
                                    <button
                                        onClick={processFiles}
                                        className="px-5 py-2 bg-white text-indigo-600 hover:text-indigo-700 rounded-xl font-semibold transition-all duration-300 hover:scale-105 hover:shadow-xl disabled:opacity-50 disabled:cursor-not-allowed"
                                        disabled={isProcessing}
                                    >
                                        {isProcessing ? 'Processando...' : '🚀 Processar Todos'}
                                    </button>
                                </div>
                            </div>
                        </div>

                        {/* Lista animada de arquivos */}
                        <div className="p-6 space-y-3">
                            {files.map((fileWithStatus, index) => {
                                const statusColors = {
                                    pending: { bg: 'bg-gray-50', border: 'border-gray-200', progress: 'from-gray-400 to-gray-500' },
                                    uploading: { bg: 'bg-blue-50', border: 'border-blue-300', progress: 'from-blue-500 to-indigo-600' },
                                    success: { bg: 'bg-green-50', border: 'border-green-300', progress: 'from-green-500 to-emerald-600' },
                                    error: { bg: 'bg-red-50', border: 'border-red-300', progress: 'from-red-500 to-pink-600' }
                                };
                                const colors = statusColors[fileWithStatus.status];

                                return (
                                    <div
                                        key={index}
                                        className="group relative animate-slide-in"
                                        style={{ animationDelay: `${index * 50}ms` }}
                                    >
                                        {/* Card premium */}
                                        <div className={`relative ${colors.bg} border-2 ${colors.border} rounded-2xl p-5 
                                                         transition-all duration-300 hover:shadow-lg hover:-translate-y-1`}>

                                            {/* Barra de progresso superior */}
                                            {fileWithStatus.status === 'uploading' && (
                                                <div className="absolute top-0 left-0 right-0 h-1 bg-gray-200 rounded-t-2xl overflow-hidden">
                                                    <div className={`h-full bg-gradient-to-r ${colors.progress} animate-pulse`}
                                                        style={{ width: '70%' }} />
                                                </div>
                                            )}

                                            <div className="flex items-start gap-4">
                                                {/* Ícone de status animado */}
                                                <div className="flex-shrink-0 mt-1">
                                                    {getStatusIcon(fileWithStatus.status)}
                                                </div>

                                                {/* Informações do arquivo */}
                                                <div className="flex-1 min-w-0">
                                                    <div className="flex items-start justify-between gap-4">
                                                        <div className="flex-1 min-w-0">
                                                            <p className="text-base font-semibold text-gray-900 truncate mb-1">
                                                                {fileWithStatus.file.name}
                                                            </p>
                                                            <div className="flex items-center gap-3 text-sm text-gray-600">
                                                                <span className="flex items-center gap-1">
                                                                    <FileText className="w-3.5 h-3.5" />
                                                                    {(fileWithStatus.file.size / 1024).toFixed(2)} KB
                                                                </span>
                                                                {fileWithStatus.file.type && (
                                                                    <span className="px-2 py-0.5 bg-gray-200 rounded-full text-xs font-medium">
                                                                        XML
                                                                    </span>
                                                                )}
                                                            </div>

                                                            {/* Resultado do processamento */}
                                                            {fileWithStatus.result && (
                                                                <>
                                                                    <div className="mt-3 flex items-center gap-2 animate-fade-in">
                                                                        <div className="px-3 py-1.5 bg-green-100 border border-green-300 rounded-lg">
                                                                            <p className="text-xs font-bold text-green-800">
                                                                                {fileWithStatus.result.document_type}
                                                                            </p>
                                                                        </div>
                                                                        <div className="flex-1 min-w-0 px-3 py-1.5 bg-white border border-gray-200 rounded-lg">
                                                                            <p className="text-xs font-mono text-gray-700 truncate">
                                                                                🔑 {fileWithStatus.result.chave_acesso}
                                                                            </p>
                                                                        </div>
                                                                        {fileWithStatus.result.duplicate && (
                                                                            <div className="px-2 py-1 bg-yellow-200 border border-yellow-400 rounded-lg">
                                                                                <span className="text-[10px] font-bold text-yellow-900">DUPLICADO</span>
                                                                            </div>
                                                                        )}
                                                                    </div>

                                                                    {/* Validações Fiscais */}
                                                                    {renderValidationBadges(fileWithStatus.result.validation)}
                                                                </>
                                                            )}

                                                            {/* Erro */}
                                                            {fileWithStatus.error && (
                                                                <div className="mt-3 px-3 py-2 bg-red-100 border border-red-300 rounded-lg animate-fade-in">
                                                                    <p className="text-xs font-medium text-red-800">
                                                                        ⚠️ {fileWithStatus.error}
                                                                    </p>
                                                                </div>
                                                            )}
                                                        </div>

                                                        {/* Botão remover */}
                                                        <button
                                                            onClick={() => removeFile(index)}
                                                            className="flex-shrink-0 p-2 text-gray-400 hover:text-red-600 hover:bg-red-100 
                                                                     rounded-lg transition-all duration-300 hover:scale-110 
                                                                     disabled:opacity-50 disabled:cursor-not-allowed"
                                                            disabled={fileWithStatus.status === 'uploading'}
                                                            title="Remover arquivo"
                                                        >
                                                            <X className="w-5 h-5" />
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                );
                            })}
                        </div>
                    </div>
                </div>
            )}
        </div>
    )
}
