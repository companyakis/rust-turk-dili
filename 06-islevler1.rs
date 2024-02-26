fn main() {

    //functions-işlevler
  
    //tanımlama snake_case ile
    //return sözcüğünü kullanmama özgürlüğümüz var
    
    //toplama işlevi
    
    fn sayilari_topla(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }
    
    //çarpma işlevi
  
    //burada println kullanalım
    //dönüş türünü belirtmedik; çünkü yazdırma yapıyoruz
    
    fn sayilari_carp(a: i32, b: i32, c: i32, d: i32) {
        println!("Sayıların çarpımı: {}", a * b * c * d)
    }
    
    //işlevleri çağıralım
    
    let toplama: i32 = sayilari_topla(-10, 1000, 245);
    println!("Toplama sonucu: {}", toplama);

    sayilari_carp(3, -11, -4, 20);
}


/*
Toplama sonucu: 1235
Sayıların çarpımı: 2640
*/

