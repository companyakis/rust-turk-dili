fn main() {
    /*
    HEAP Memory

    Ödünç Alma Borrowing

    Clone ve move olmadan, pointer ile dayanak gösterebilirme (referans göstermek) 

    & ile veriye referans gösteriyoruz. Adrese vurgu yapmak gibi düşünebiliriz.

    Verinin değişebilir (mutable) olup olmadığına dikkat edelim
     */

    //mutable borrowing örneği

    //Ödünç alan değişken veriyi kullandıktan sonra, ana değişken verinin sahibi olur

    let mut gunaydin_de = String::from("Günaydın Türkiye");

    let soz1 = &mut gunaydin_de;

    soz1.push_str(" yeni umutlara!");

    println!("soz1 değişkeni: {}", soz1); //soz1 değişkeni: Günaydın Türkiye yeni umutlara!

    println!("gunaydin_de değişkeni: {}", gunaydin_de); //gunaydin_de değişkeni: Günaydın Türkiye yeni umutlara!

}
