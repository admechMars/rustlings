// referencia2
//
// Digite "dica" (sem as aspas) no terminal para receber uma dica

// EU NÃO ESTOU PRONTO

fn main() {
    let mut texto = "Salve";
    // Modifique apenas a linha abaixo
    let texto2 = &texto;

    *texto2 = "Olá";

    println!("{} Mundo", texto)
}