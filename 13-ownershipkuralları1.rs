fn main() {

/*
Rust dilinde bellek, derleyicinin kontrol ettiği sahiplik (ownership) kuralları aracılığıyla yönetilir.

Ownership kuralları özetle:
1- Her değerin bir sahibi ( owner ) vardır
2- Bir değerin aynı anda yalnızca bir sahibi olabilir
3- Veriyi yalnızca sahibi kullanabilir
4- Sahip kapsam dışına çıktığında, değer RAM'den silinir

Bu kuralların amacı HEAP memory'deki verilerin yönetimini kontrol etme...

STACK memory

Sabit boyutlu veriler saklanır
HEAP kısmına veri eklemekten daha hızlıdır
Veriler sırayla üst üste eklenir
Veriler, bellekten çıkartılırken, en üstten sırayla çıkartılır.
Son giren eleman ilk çıkartılacak elemandır (LIFO)


*/

let a = 10;
let b  = a;
let c = 15;

//Bir sonraki bölümde LIFO'yu (son giren ilk çıkartılacaktır) örneklendirelim

}

