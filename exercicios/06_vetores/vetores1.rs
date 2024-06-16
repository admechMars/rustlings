// vetores1
//
// Digite "dica" (sem as aspas) no terminal para receber uma dica


fn lista() -> Vec<&'static str> {
    // Adicione "Tomate", "Macarrão" e "Vinagre" a lista
    let lista_compras = vec!["Tomate", "Macarrão","Vinagre"];


    // Não modifique essa linha
    lista_compras
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_similarity() {
        let lista_compras = lista();

        assert_eq!(lista_compras, vec!["Tomate", "Macarrão", "Vinagre"])
    }
}