// se1
//
// Digite "dica" (sem as aspas) no terminal para receber uma dica

// EU NÃO ESTOU PRONTO

pub fn maior(a: i32, b: i32) -> bool {
    // Retorne `true` se o número 'a' for maior que o número 'b'
    // usando condicional (if)

}

// Ignore isso por enquanto :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(true, maior(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(false, maior(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(false, maior(42, 42));
    }
}