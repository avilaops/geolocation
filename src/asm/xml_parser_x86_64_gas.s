# xml_parser_x86_64_gas.s
# Funções Assembly otimizadas para processamento de XML (GAS syntax para Linux/macOS)
# Arquitetura: x86_64

.intel_syntax noprefix
.text

# Função: fast_char_validate
# Valida se um caractere é válido em XML
# Parâmetros: RDI = ponteiro para buffer, RSI = tamanho
# Retorno: RAX = 1 se válido, 0 se inválido
.globl fast_char_validate
.type fast_char_validate, @function
fast_char_validate:
    push rbx
    push r12
    push r13
    
    xor rax, rax
    test rsi, rsi
    jz .Ldone
    
    mov rbx, rdi
    mov r12, rsi
    
.Lvalidate_loop:
    movzx r13, byte ptr [rbx]
    
    cmp r13b, 0x09
    je .Lvalid_char
    cmp r13b, 0x0A
    je .Lvalid_char
    cmp r13b, 0x0D
    je .Lvalid_char
    cmp r13b, 0x20
    jl .Linvalid_char
    cmp r13b, 0x7E
    jle .Lvalid_char
    
    cmp r13b, 0xC0
    jl .Linvalid_char
    
.Lvalid_char:
    inc rbx
    dec r12
    jnz .Lvalidate_loop
    
    mov rax, 1
    jmp .Ldone
    
.Linvalid_char:
    xor rax, rax
    
.Ldone:
    pop r13
    pop r12
    pop rbx
    ret


# Função: fast_find_tag
# Busca rapidamente por uma tag XML no buffer
# Parâmetros: RDI = buffer, RSI = tamanho, RDX = tag, RCX = tamanho da tag
# Retorno: RAX = posição da tag ou -1 se não encontrada
.globl fast_find_tag
.type fast_find_tag, @function
fast_find_tag:
    push rbx
    push r12
    push r13
    push r14
    push r15
    
    mov rax, -1
    test rsi, rsi
    jz .Lfind_done
    
    mov rbx, rdi
    mov r12, rsi
    mov r13, rdx
    mov r14, rcx
    
    sub r12, r14
    jl .Lfind_done
    
    xor r15, r15
    
.Lsearch_loop:
    mov al, byte ptr [rbx + r15]
    cmp al, '<'
    jne .Lnext_pos
    
    mov rdi, r13
    lea rsi, [rbx + r15 + 1]
    mov rdx, r14
    call compare_bytes
    test rax, rax
    jnz .Lfound_tag
    
.Lnext_pos:
    inc r15
    cmp r15, r12
    jle .Lsearch_loop
    
    mov rax, -1
    jmp .Lfind_done
    
.Lfound_tag:
    mov rax, r15
    
.Lfind_done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret


# Função: fast_extract_number
# Extrai número de string ASCII de forma otimizada
# Parâmetros: RDI = ponteiro string, RSI = tamanho max
# Retorno: RAX = número extraído
.globl fast_extract_number
.type fast_extract_number, @function
fast_extract_number:
    push rbx
    push r12
    
    xor rax, rax
    xor rbx, rbx
    mov r12, rsi
    
.Lextract_loop:
    test r12, r12
    jz .Lextract_done
    
    movzx rsi, byte ptr [rdi]
    
    cmp sil, '0'
    jl .Lextract_done
    cmp sil, '9'
    jg .Lextract_done
    
    sub sil, '0'
    
    imul rbx, rbx, 10
    movzx rsi, sil
    add rbx, rsi
    
    inc rdi
    dec r12
    jmp .Lextract_loop
    
.Lextract_done:
    mov rax, rbx
    pop r12
    pop rbx
    ret


# Função auxiliar: compare_bytes
.type compare_bytes, @function
compare_bytes:
    push rbx
    push r12
    
    xor rax, rax
    test rdx, rdx
    jz .Lcmp_equal
    
    mov rbx, rdx
.Lcmp_loop:
    mov al, byte ptr [rdi]
    mov r12b, byte ptr [rsi]
    cmp al, r12b
    jne .Lcmp_different
    inc rdi
    inc rsi
    dec rbx
    jnz .Lcmp_loop
    
.Lcmp_equal:
    mov rax, 1
    jmp .Lcmp_done
    
.Lcmp_different:
    xor rax, rax
    
.Lcmp_done:
    pop r12
    pop rbx
    ret

.att_syntax prefix
