// referencia2
//
// Digite "dica" (sem as aspas) no terminal para receber uma dica


fn main() {
    let mut texto = "Salve";
    // Modifique apenas a linha abaixo
    let texto2 = &mut texto;

    *texto2 = "Olá";

    println!("{} Mundo", texto)
}