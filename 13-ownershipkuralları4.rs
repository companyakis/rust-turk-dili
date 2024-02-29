fn main() {

    /*
    HEAP Memory

    Ownership için belirlenen katı kurallardan kaçınabilmenin yolu yok mu?
    Var, ama bellek yönetiminden kaçınmak her zaman iyi sonuçlar vermeyebilir

    clone() işlevi!
 
     */

    let string_1 = String::from("Günaydın!");

    println!("{}", string_1); //Günaydın!

    //clone()

    let string_2 = string_1.clone();

    println!("{}", string_2); //Günaydın!

    println!("{}", string_1); //Günaydın!
}

