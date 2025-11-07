#[cfg(build_with_asm)]
mod implementation {
    // Interface para funções Assembly
    extern "C" {
        /// Valida se os caracteres em um buffer são válidos para XML
        /// Retorna 1 se válido, 0 caso contrário
        fn fast_char_validate(buffer: *const u8, size: usize) -> i32;

        /// Busca rapidamente por uma tag XML no buffer
        /// Retorna a posição da tag ou -1 se não encontrada
        fn fast_find_tag(
            buffer: *const u8,
            buffer_size: usize,
            tag: *const u8,
            tag_size: usize,
        ) -> i64;

        /// Extrai um número de uma string ASCII de forma otimizada
        /// Retorna o número extraído
        fn fast_extract_number(buffer: *const u8, max_size: usize) -> u64;
    }

    /// Wrapper seguro para validação de caracteres
    pub fn validate_xml_chars(data: &[u8]) -> bool {
        if data.is_empty() {
            return true;
        }
        unsafe { fast_char_validate(data.as_ptr(), data.len()) == 1 }
    }

    /// Wrapper seguro para busca de tags
    pub fn find_tag(buffer: &[u8], tag: &str) -> Option<usize> {
        if buffer.is_empty() || tag.is_empty() {
            return None;
        }

        let result = unsafe {
            fast_find_tag(buffer.as_ptr(), buffer.len(), tag.as_ptr(), tag.len())
        };

        if result >= 0 {
            Some(result as usize)
        } else {
            None
        }
    }

    /// Wrapper seguro para extração de números
    pub fn extract_number(data: &[u8]) -> Option<u64> {
        if data.is_empty() {
            return None;
        }

        let result = unsafe { fast_extract_number(data.as_ptr(), data.len()) };

        if result > 0 {
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(build_without_asm)]
mod implementation {
    /// Implementação pura em Rust para validação de caracteres XML básicos.
    pub fn validate_xml_chars(data: &[u8]) -> bool {
        data.iter().all(|&b| matches!(b, 0x09 | 0x0A | 0x0D | 0x20..=0x7E))
    }

    /// Implementação simples de busca de tag utilizando varredura por janela.
    pub fn find_tag(buffer: &[u8], tag: &str) -> Option<usize> {
        let tag_bytes = tag.as_bytes();
        if tag_bytes.is_empty() {
            return None;
        }

        let mut needle = Vec::with_capacity(tag_bytes.len() + 1);
        needle.push(b'<');
        needle.extend_from_slice(tag_bytes);

        buffer
            .windows(needle.len())
            .position(|window| window == needle)
    }

    /// Extrai número do início da string, abortando se primeiro byte não for dígito.
    pub fn extract_number(data: &[u8]) -> Option<u64> {
        let mut iter = data.iter();
        let first = *iter.next()?;

        if !first.is_ascii_digit() {
            return None;
        }

        let mut value = (first - b'0') as u64;

        for &byte in iter {
            if !byte.is_ascii_digit() {
                break;
            }

            value = value
                .checked_mul(10)?
                .checked_add((byte - b'0') as u64)?;
        }

        Some(value)
    }
}

pub use implementation::{extract_number, find_tag, validate_xml_chars};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_xml_chars() {
        let valid = b"Hello World <tag>content</tag>";
        assert!(validate_xml_chars(valid));
        
        let empty: &[u8] = &[];
        assert!(validate_xml_chars(empty));
    }

    #[test]
    fn test_find_tag() {
        let xml = b"<root><nfeProc><infNFe>data</infNFe></nfeProc></root>";
        assert_eq!(find_tag(xml, "nfeProc"), Some(6));
        assert_eq!(find_tag(xml, "infNFe"), Some(15));
        assert_eq!(find_tag(xml, "notfound"), None);
    }

    #[test]
    fn test_extract_number() {
        let num = b"12345abc";
        assert_eq!(extract_number(num), Some(12345));
        
        let no_num = b"abc123";
        assert_eq!(extract_number(no_num), None);
    }
}
