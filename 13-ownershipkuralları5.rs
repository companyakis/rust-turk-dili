fn main() {
    /*
    HEAP Memory

    Kapsam (scope) konusunu örneklendirelim

     */

    {
        let string_1 = String::from("Günaydın!");

        println!("{}", string_1); //Günaydın!
    }

    //Değişken burada geçerli değil

    println!("{}", string_1); //"not found in this scope" hatası aldık
}
