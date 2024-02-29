fn main() {

//LIFO'yu örneklendirelim. Son giren, bellekten ilk çıkartılır
//Bir kaba üst üste nesneler eklendiğinde, en yukardakini önce alma gibi...
//RAM Stack için kuramsal bir çerçeve oluşturalım, veriler yukarıdan aşağıya doğru sıralandı gibi düşünelim

let a = 10; //önce a değişkeni tanımlandı ve değer atandı

/*
kuramsal_bellek_adresi  değişken_adı    değer
0                       a               10

*/

let b  = a; //b değişkeni sahaya indi

/*
kuramsal_bellek_adresi  değişken_adı    değer
1                       b               10
0                       a               10

*/

let c = 15; //c değişkeni tanımlandı ve değer atandı

/*
kuramsal_bellek_adresi  değişken_adı    değer
2                       c               15
1                       b               10
0                       a               10

*/



}

