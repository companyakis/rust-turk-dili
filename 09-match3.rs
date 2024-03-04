fn main() {

    //birden fazla değeri denetleme örneği yapalım

    let gun: u8 = 7;

    match gun {

        1..=5 => println!("Hafta içi bir gündeyiz."),
        6..=7 => println!("Hafta sonu bir gündeyiz."),
        _ => println!("Lütfen, gün sayısını 1 ile 7 arasında belirtiniz!"),
     
    }
 
}

//Hafta sonu bir gündeyiz.
