#[allow(unused_variables)]


fn main() {

    //tür belirtmeden kullanım yapabiliriz, ama ben belirterek kullanıyorum
    
    //bool true/false
    let mutlu_muyuz: bool = true;
    println!("mutlu_muyuz: {}", mutlu_muyuz);
    
    let uyanik_misin = false; //tür belirtmedik
    println!("uyanik_misin: {}", uyanik_misin);
    
    //sıfırdan küçük olmayan sayılar u8 u16 u32 u64 u128
    let mut dolar_kuru: u8 = 30;
    println!("dolar_kuru: {}", dolar_kuru);
    
    let gelecek_yil: u16 = 2025;
    println!("gelecek_yil: {}", gelecek_yil);
    
    //sıfırdan küçük olarak da kullanabileceklerimiz i8 i16 i32 i64 i128
    let sayi_1: i8 = -21;
    let sayi_2: i16 = 1000; //sıfırdan büyük burada!
    println!("sayi_1: {}", sayi_1);
    println!("sayi_2: {}", sayi_2);
    
    //ondalıklı f32 f64
    let senin_eth: f32 = 25.46;
    println!("senin_eth: {}", senin_eth);
    
    //karakter '' ile
    let adimin_ilki: char = 'm';
    
    //cümle, sözcük "" ile
    let ben: &str = "Mustafa Büyükdereli";
    println!("Ben: {}", ben);
 
    //tuple ile farklı türden veriler, birlikte örnekleyelim
    let onun_bilgileri: (String, String, u8, bool) = ("Aygün".to_string(), "Derelioglu".to_string(), 34, true);
    println!("Adı: {}", onun_bilgileri.0);
    println!("Soyadı: {}", onun_bilgileri.1);
    println!("Yaşı: {}", onun_bilgileri.2);
    println!("Evli mi: {}", onun_bilgileri.3);
}


/*
mutlu_muyuz: true
uyanik_misin: false
dolar_kuru: 30
gelecek_yil: 2025
sayi_1: -21
sayi_2: 1000
senin_eth: 25.46
Ben: Mustafa Büyükdereli
Adı: Aygün
Soyadı: Derelioglu
Yaşı: 34
Evli mi: true
*/

