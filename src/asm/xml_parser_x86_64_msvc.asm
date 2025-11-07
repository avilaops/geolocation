; xml_parser_x86_64_msvc.asm
; Funções Assembly otimizadas para processamento de XML (MASM syntax para Windows)
; Arquitetura: x86_64

_TEXT SEGMENT

; Função: fast_char_validate
; Valida se um caractere é válido em XML
; Parâmetros: RCX = ponteiro para buffer, RDX = tamanho
; Retorno: RAX = 1 se válido, 0 se inválido
PUBLIC fast_char_validate
fast_char_validate PROC
    push rbx
    push r12
    push r13
    
    xor rax, rax
    test rdx, rdx
    jz done
    
    mov rbx, rcx
    mov r12, rdx
    
validate_loop:
    movzx r13, byte ptr [rbx]
    
    cmp r13b, 09h
    je valid_char
    cmp r13b, 0Ah
    je valid_char
    cmp r13b, 0Dh
    je valid_char
    cmp r13b, 20h
    jl invalid_char
    cmp r13b, 7Eh
    jle valid_char
    
    cmp r13b, 0C0h
    jl invalid_char
    
valid_char:
    inc rbx
    dec r12
    jnz validate_loop
    
    mov rax, 1
    jmp done
    
invalid_char:
    xor rax, rax
    
done:
    pop r13
    pop r12
    pop rbx
    ret
fast_char_validate ENDP


; Função: fast_find_tag
; Busca rapidamente por uma tag XML no buffer
; Parâmetros: RCX = buffer, RDX = tamanho, R8 = tag procurada, R9 = tamanho da tag
; Retorno: RAX = posição da tag ou -1 se não encontrada
PUBLIC fast_find_tag
fast_find_tag PROC
    push rbx
    push r12
    push r13
    push r14
    push r15
    
    mov rax, -1           ; resultado padrão = não encontrado
    test rdx, rdx
    jz find_done
    
    mov rbx, rcx          ; rbx = buffer
    mov r12, rdx          ; r12 = tamanho buffer
    mov r13, r8           ; r13 = tag
    mov r14, r9           ; r14 = tamanho tag
    
    sub r12, r14          ; ajusta limite de busca
    jl find_done
    
    xor r15, r15          ; r15 = índice atual
    
search_loop:
    mov al, byte ptr [rbx + r15]
    cmp al, 3Ch
    jne next_pos
    
    mov rcx, r13
    lea rdx, [rbx + r15 + 1]
    mov r8, r14
    call compare_bytes
    test rax, rax
    jnz found_tag
    
next_pos:
    inc r15
    cmp r15, r12
    jle search_loop
    
    mov rax, -1
    jmp find_done
    
found_tag:
    mov rax, r15
    
find_done:
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    ret
fast_find_tag ENDP


; Função auxiliar: compare_bytes
; Compara dois buffers de bytes
; RCX = buffer1, RDX = buffer2, R8 = tamanho
; Retorno: RAX = 1 se iguais, 0 se diferentes
PUBLIC compare_bytes
compare_bytes PROC
    push rbx
    push r12
    
    xor rax, rax
    test r8, r8
    jz cmp_equal
    
    mov rbx, r8
cmp_loop:
    mov al, byte ptr [rcx]
    mov r12b, byte ptr [rdx]
    cmp al, r12b
    jne cmp_different
    inc rcx
    inc rdx
    dec rbx
    jnz cmp_loop
    
cmp_equal:
    mov rax, 1
    jmp cmp_done
    
cmp_different:
    xor rax, rax
    
cmp_done:
    pop r12
    pop rbx
    ret
compare_bytes ENDP


; Função: fast_extract_number
; Extrai número de string ASCII de forma otimizada
; Parâmetros: RCX = ponteiro string, RDX = tamanho max
; Retorno: RAX = número extraído
PUBLIC fast_extract_number
fast_extract_number PROC
    push rbx
    push r12
    
    xor rax, rax          ; resultado = 0
    xor rbx, rbx          ; acumulador
    mov r12, rdx          ; contador
    
extract_loop:
    test r12, r12
    jz extract_done
    
    movzx rdx, byte ptr [rcx]
    
    cmp dl, 30h
    jl extract_done
    cmp dl, 39h
    jg extract_done
    
    sub dl, 30h
    
    ; resultado = resultado * 10 + dígito
    imul rbx, rbx, 10
    movzx rdx, dl
    add rbx, rdx
    
    inc rcx
    dec r12
    jmp extract_loop
    
extract_done:
    mov rax, rbx
    pop r12
    pop rbx
    ret
fast_extract_number ENDP

_TEXT ENDS

END
