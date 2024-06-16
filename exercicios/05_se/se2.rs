// se2
//
// Digite "dica" (sem as aspas) no terminal para receber uma dica


// Faça uma condicional para:
// Se palvra for igual a fizz retorne "foo"
// Se 'palavra' for igual a fuzz retorne "bar"
// Se não retorne "baz"
pub fn foo_if_fizz(palavra: &str) -> &str {
    if palavra == "fizz" {
        "foo"
    } else if palavra ==  "fuzz" {
        "bar"
    }else{
        "baz"
    }
}

// Ignore isso por enquanto :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}