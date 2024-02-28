fn main() {

    //dizi parçalama, array slicing

    let sayilar: [i8; 6] = [-3, -2, -1, 0, 1, 2];

    //ilk elemanı yazdıralım

    let ilk_oge = sayilar[0];

    println!("{:?}", ilk_oge); //-3

    //eksi değerleri almaya çalışalım

    //Rust çilesi başlıyor:) Sahiplik uyarıları var
    //let eksi_degerliler = sayilar[0..3]; //bu kullanım hata verecek

    let eksi_degerliler = &sayilar[0..3]; //ödünçlik konusu

    println!("{:?}", eksi_degerliler); //[-3, -2, -1]

}

