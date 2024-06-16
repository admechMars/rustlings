// referencia3
//
// Faça o código funcionar apenas re-ordenando as linhas
// sem adicionar, editar ou remover qualquer uma delas
//
// Digite "dica" (sem as aspas) no terminal para receber uma dica


#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;

    assert_eq!(x, 1200);
}