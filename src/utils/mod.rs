/// Utilitários gerais para o sistema
use crate::error::{GeolocationError, Result};

/// Valida um CNPJ
pub fn validate_cnpj(cnpj: &str) -> bool {
    let cnpj: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();

    if cnpj.len() != 14 {
        return false;
    }

    // Verifica se todos os dígitos são iguais
    if cnpj.chars().all(|c| c == cnpj.chars().next().unwrap()) {
        return false;
    }

    // Calcula os dígitos verificadores
    let digits: Vec<u32> = cnpj.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // Primeiro dígito verificador
    let weights1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum1: u32 = digits[..12]
        .iter()
        .zip(weights1.iter())
        .map(|(d, w)| d * w)
        .sum();
    let dv1 = match sum1 % 11 {
        0 | 1 => 0,
        n => 11 - n,
    };

    if digits[12] != dv1 {
        return false;
    }

    // Segundo dígito verificador
    let weights2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let sum2: u32 = digits[..13]
        .iter()
        .zip(weights2.iter())
        .map(|(d, w)| d * w)
        .sum();
    let dv2 = match sum2 % 11 {
        0 | 1 => 0,
        n => 11 - n,
    };

    digits[13] == dv2
}

/// Valida um CPF
pub fn validate_cpf(cpf: &str) -> bool {
    let cpf: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    if cpf.len() != 11 {
        return false;
    }

    // Verifica se todos os dígitos são iguais
    if cpf.chars().all(|c| c == cpf.chars().next().unwrap()) {
        return false;
    }

    let digits: Vec<u32> = cpf.chars().map(|c| c.to_digit(10).unwrap()).collect();

    // Primeiro dígito verificador
    let sum1: u32 = digits[..9]
        .iter()
        .enumerate()
        .map(|(i, d)| d * (10 - i as u32))
        .sum();
    let dv1 = match (sum1 * 10) % 11 {
        10 => 0,
        n => n,
    };

    if digits[9] != dv1 {
        return false;
    }

    // Segundo dígito verificador
    let sum2: u32 = digits[..10]
        .iter()
        .enumerate()
        .map(|(i, d)| d * (11 - i as u32))
        .sum();
    let dv2 = match (sum2 * 10) % 11 {
        10 => 0,
        n => n,
    };

    digits[10] == dv2
}

/// Formata um CNPJ para exibição
pub fn format_cnpj(cnpj: &str) -> String {
    let cnpj: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();

    if cnpj.len() != 14 {
        return cnpj;
    }

    format!(
        "{}.{}.{}/{}-{}",
        &cnpj[0..2],
        &cnpj[2..5],
        &cnpj[5..8],
        &cnpj[8..12],
        &cnpj[12..14]
    )
}

/// Formata um CPF para exibição
pub fn format_cpf(cpf: &str) -> String {
    let cpf: String = cpf.chars().filter(|c| c.is_ascii_digit()).collect();

    if cpf.len() != 11 {
        return cpf;
    }

    format!(
        "{}.{}.{}-{}",
        &cpf[0..3],
        &cpf[3..6],
        &cpf[6..9],
        &cpf[9..11]
    )
}

/// Formata uma chave de acesso para exibição
pub fn format_chave_acesso(chave: &str) -> String {
    if chave.len() != 44 {
        return chave.to_string();
    }

    let mut formatted = String::new();
    for (i, c) in chave.chars().enumerate() {
        if i > 0 && i % 4 == 0 {
            formatted.push(' ');
        }
        formatted.push(c);
    }
    formatted
}

pub mod metrics;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cnpj() {
        assert!(validate_cnpj("11222333000181"));
        assert!(validate_cnpj("11.222.333/0001-81"));
        assert!(!validate_cnpj("11222333000180"));
        assert!(!validate_cnpj("11111111111111"));
    }

    #[test]
    fn test_validate_cpf() {
        assert!(validate_cpf("11144477735"));
        assert!(validate_cpf("111.444.777-35"));
        assert!(!validate_cpf("11144477736"));
        assert!(!validate_cpf("11111111111"));
    }

    #[test]
    fn test_format_cnpj() {
        assert_eq!(format_cnpj("11222333000181"), "11.222.333/0001-81");
    }

    #[test]
    fn test_format_cpf() {
        assert_eq!(format_cpf("11144477735"), "111.444.777-35");
    }

    #[test]
    fn test_format_chave_acesso() {
        let chave = "35210112345678901234567890123456789012345678";
        let formatted = format_chave_acesso(chave);
        assert!(formatted.contains(' '));
        assert_eq!(formatted.split_whitespace().count(), 11);
    }
}
