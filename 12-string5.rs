fn main() {

    //String parçalama

    let ad = String::from("Mustafa"); //tür belirtmedik

    let soyad: String = "Büyükdereli".to_string();

    let usta = &ad[1..5]; //son sayı alınmıyor! Python'da [1:5]

    println!("Usta mı oldunuz: {}", usta); //Usta mı oldunuz: usta
}

