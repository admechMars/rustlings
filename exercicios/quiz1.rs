// quiz1.rs
//
// Este é um teste para as seções a seguir:
// - Variáveis
// - Funções
// - Se
//
// Maria está comprando maçãs. O preço de uma maçã é calculado da seguinte forma:
// - Uma maçã custa 2 vbucks.
// - Se Mary comprar mais de 40 maçãs, cada maçã custará apenas 1 vbuck!
// Escreva uma função que calcule o preço de um pedido de maçãs dada a
// quantidade comprada.
//
// Sem dicas dessa vez ;)


// Escreva sua função abaixo
// fn calcula_preco {



// Não modifique as linhas a baixo
#[test]
fn verify_test() {
    let price1 = calcula_preco(35);
    let price2 = calcula_preco(40);
    let price3 = calcula_preco(41);
    let price4 = calcula_preco(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}

fn calcula_preco(qnt:i32)->i32{
    if qnt>40{
        qnt
    }else{
        qnt * 2
    }
}