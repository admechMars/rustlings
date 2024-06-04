// vetores2
//
// Digite "dica" (sem as aspas) no terminal para receber uma dica

// EU NÃO ESTOU PRONTO

fn lista() -> Vec<&'static str> {
    // Não modifique a linha abaixo
    let mut lista_compras = vec!["Tomate", "Macarrão", "Vinagre"];

    // Adicione "Batata" e "Molho" a lista de compras

    lista_compras
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_similarity() {
        let lista_compras = lista();

        assert_eq!(lista_compras, vec!["Tomate", "Macarrão", "Vinagre", "Batata", "Molho"])
    }
}