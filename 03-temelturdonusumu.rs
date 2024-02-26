#[allow(unused_variables)]


fn main() {

    let sayi_1: u8 = 22;
    
    let sayi_2: u16 = 4444;
    
    //let toplam = sayi_1 + sayi_2;
    
    //Rust gıcık bir dildir. Bu sayıları toplamaya izin vermez:)
    //println!("Toplam: {}", toplam); //no implementation for `u8 + u16
    
    //as ile dönüşüm yapmalıyız
    
    let toplam = sayi_1 as u16 + sayi_2;
    
    println!("Toplam: {}", toplam); //Toplam: 4466

}


/*

*/

