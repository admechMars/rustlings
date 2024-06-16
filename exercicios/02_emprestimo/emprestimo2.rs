// emprestimo2.rs
//
// Digite "dica" (sem as aspas) no terminal para receber uma dica


fn main() {
    let texto = "Bom".to_string();
    // Modifique somente a linha a baixo
    let texto2 = texto.clone();

    println!("{} {}", texto, texto2)
}