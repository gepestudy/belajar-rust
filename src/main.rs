
/*TOLONG BACA DARI BAWAH KE ATAS. */


fn main() {
    println!("Hello, world!");
}

// ########################################
// ! MATCHING PATTER VALUE
#[test]
fn matching_value() {
    // ini sama aja kaya switch case atau if else aja, jd ini sepertinya jarang di pake nantinya
    let name = "pamungkas"; // ganti value ini untuk cek

    match name {
        "ilham"=>{ // ktia bisa juga pake pipe, eg: "iham" | "pamungkas"
            println!("hello , {}", name);
        }
        "gilang"=>{
            println!("hello , {}", name);
        }
        other => {
            println!("hello other, {}", other);
        }
    }

    let value = 1000;

    match value {
        0..=100 => {
            println!("{} is between 0 100", value);
        }
        other =>{
            println!("{} is between 100 unlimited", other);
        }
    }

    let point = GeoPoint(-6.20000, 106.81667);
    match point {
        GeoPoint(long, -0.1) => { // jika lat nya 0.1 maka ini terpilih
            println!("long: {}", long);
        }
        GeoPoint(0.0,lat) =>{ // jika long nya 0.0 ini terpilih
            println!("lat: {}", lat);
        }
        GeoPoint(long, lat) => { // verapapun
            println!("long: {}, lat: {}", long, lat);
        }
    }

    // ! destructuiring struct pattern
    let person = Person{
        first_name: String::from("ilham"),
        last_name: String::from("gilang"),
        age: 26,
    };
    // khusus untuk tuple struct atau enum, kita tidak bisa pakai .. karna tidak ada namanya kaya struct. tp pake _ aja untuk mengabaikan
    match person {
        Person {first_name,age,..}=>{ // jika kita hanya butuh first_name dan age, sisanya harus pakai ".."
            println!("first name: {}, age: {}", first_name, age);
        }
        Person {first_name,..}=>{ // jika kita hanya butuh first_name dan age, sisanya harus pakai ".."
            println!("first name: {}", first_name);
        }
    }
    
}
// ########################################


// ########################################
// ! PATERN MATCHING
/**
 * ini biasanya digunakan unutk if else enum. karna enum tidak support if else, kita tidak bisa akses datanya
 * ini di gunakan untuk apapun ini sangat kompleks bisa buat apa aja, ambil nama variabel, value dll. tp saya mules
 */

#[test]
fn test_matching_enum() {
    let level: Level = Level::Platinum;

    match level { // wajib cover semua yang ada di level.
        Level::Reguler =>{ // eksekusi apapun
            println!("reguler");
        }
        Level::Premium => {
            println!("premium");
        }
        Level::Platinum => {
            println!("platinum");
        }
    }

    let payment :Payment = Payment::BankTransfer(String::from("BCA"), String::from("1234567890"));
    payment.pay(15000);
}

// ########################################


// ########################################
// ! ENUM

enum Level {
    Reguler,
    Premium,
    Platinum
}
enum Payment {
    /** 
     * ! menariknya enum di rust bisa begini. kaya struct versi low budget
     * ! lebih menarik lg ternyata enum bisa punya method wokwowkowk
     */
    // card number
    CreaditCard(String),
    // bank name, account number
    BankTransfer(String, String),
    // ewallet name, ewallet number
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) { // buat contoh patern matching, yg sebelumnya di timpa soalnya cuma dikit
        match self {
            Self::CreaditCard(card_number) => {
                println!("paid {} with card number {}", amount,card_number);
            }
            Self::BankTransfer(bank_name, account_number) => {
                println!("paid {} with {} account number {}", amount,bank_name, account_number);
            }
            Self::EWallet(ewallet_name, ewallet_number) => {
                println!("paid {} with {} ewallet number {}", amount,ewallet_name, ewallet_number);
            }
        }
    }
}

#[test]
fn test_enum() {
    let _level:Level = Level::Reguler;
    let _level2:Level = Level::Premium;
    let _level3:Level = Level::Platinum;

    let _payment:Payment = Payment::CreaditCard(String::from("1234567890"));
    let _payment2:Payment = Payment::BankTransfer(String::from("BRI"), String::from("1234567890"));
    let _payment3:Payment = Payment::EWallet(String::from("GOPAY"), String::from("1234567890"));

    // enum method
    _payment.pay(10000);
}
// ########################################

// ########################################
// ! ASSOCIATED FUNCTION
/**
 * assoc function itu kaya static function di php. jadi dia ada di bawah suatu struct/tipe data lain.
 * cuman dia ga butuh self, yang menjadikan dia bukan method.
 * cara panggilnya any::assoc_fn()
 */
impl GeoPoint { // struct GeoPoint ada di bawah
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn test_assoc_function() {
    let geo_point = GeoPoint::new(-6.20000, 106.81667);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

// ! METHOD
/** 
 * method ya method wkwkw. tp disini ad ayg baru yaitu self. 
 * self itu kaya `this`. jd bisa akses ke properti/field dari struct itu sendiri.
 * kenapa pake reference, ya emang lu mau nanti field2nya pindah owner ketika lagi di pinjem si self
*/
struct Person2 {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person2 {
    fn hello(&self) {
        println!("hello {} {} {}", self.first_name, self.last_name, self.age);
    }
}

#[test]
fn test_method() {
    let person: Person2 = Person2{
        age: 26,
        first_name: String::from("ilham"),
        last_name: String::from("gilang"),
    };

    person.hello();
}
// ########################################


// ########################################
// ! TUPLE STRUCT
/**
 * mirip kaya tuple. ngambil datanya juga kaya tuple. cuman bedanya ini kaya tuple tp di namain.
 * gunain tuple biasa ajalah
 */
struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(-1.0, 1.0);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}
// ! STRUCT
/**
 * struct di rust mirip struct di golang, btw ini kan ga fixed size(feeling aja wkwk) jd dia di heap di simpanya
 */

 #[derive(Debug)] // ! kalo mau println harus pake ini, karna struct pada dasarnya tidak implement debug trait/interface
 struct Person {
    first_name: String,
    last_name: String,
    age: u8,
 }

//  kita juga bisa jadikan struct sebagai paramter / return value
fn print_person(person: &Person) { // pake reference aja biar ownershipnya ga pindah
    println!("person is {:#?}", person);
}

 #[test]
 fn struct_person() {
    let person: Person = Person{
        first_name: String::from("ilham"),
        last_name: String::from("gilang"),
        age: 26,
    };

    print_person(&person);

    let mut person2: Person = Person{..person}; // !ini akan memindahkan owner jika ada field yang bertipe dinamis (heap)

    person2.last_name = String::from("pamungkas");
    // println!("person {:#?}", person); // ! ini ga bisa karna first/last name itu udah pindah owner
    print_person(&person2);

    let person3: Person = Person{
        first_name: person2.first_name.clone(), // kita pake clone untuk emgnhindari pepindahan owner
        last_name: person2.last_name.clone(),
        age: person2.age, // ini gausah karna u8 itu implement trait copy alias dia ada di stack
    };
    print_person(&person3);
    
 }
// ########################################


// ########################################
// ! SLICE
/** 
 * slice itu adalah potongan dari array atau apapun, jadi dia pasti reference, karna kepilikanya bukan punya dia
 */
#[test]
fn slice_reference() {
    let array : [i32; 11] = [0,1,2,3,4,5,6,7,8,9,10];

    let slice = &array[..]; // ? ini adalah slice
    println!("full array &array[..] is {:?}", slice);

    let slice2 = &array[..5];
    println!("slice2 &array[..5] is {:?}", slice2);

    let slice3 = &array[1..];
    println!("slice3 &array[1..] is {:?}", slice3);

    // ini contoh dari docs nya
    let arr = [0, 1, 2, 3, 4];
    assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]);
    assert_eq!(arr[ .. 3], [0, 1, 2      ]);
    assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
    assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
    assert_eq!(arr[1.. 3], [   1, 2      ]); // This is a `Range`
    assert_eq!(arr[1..=3], [   1, 2, 3   ]);
}
// ########################################


// ########################################
// dangling pointer / null reference 
/**
 * * good case
 * kita sudah tau jangan mereturn reference karna itu akan sia-sia dan value akan hilang karna owner di hapus
 * jadi kita pindahkan aja ownernya ke penampung function itu, jd hanya return value biasa aja
 * karna ini di heap ya dia pindah owner nya otomatis,
 */
fn get_full_name(first_name: &String,last_name: &String)-> String {
    let name = format!("{} {}", first_name, last_name);
    return name
}

#[test]
fn test_dangling_pointer() {
    let first_name = String::from("ilham");
    let last_name = String::from("gilang");

    let full_name = get_full_name(&first_name, &last_name); // ? kita tjadikan variable baru untuk owner dari return functionya
    println!("full name is {}", full_name);
    // ini bukan case di atas sih. tp biar ga lupa, kita masih bisa akses first_name dan last_name
    // karna yang kita kirim itu reference, jd ga pindah owner
    println!("first name is {}", first_name);
    println!("last name is {}", last_name);
}

/** 
 * ! bad case
 * kita ga bisa kaya golang yang mereturn reference/pointer, karna golang kan otomatis di atur GC,
 * sedangkan rust itu owner lifecycle nya otomatis tp berdasarkan eksekusi(jika sudah selesai maka di hapus)
 * jadi ketika kita return &string, kan kita return &name, ketika function selesai maka owner name akan di hapus
 * begitu juga isinya, jd &string itu tidak ada isinya jadinya error.
 */
// fn get_full_name(first_name: &String,last_name: &String)-> &String {
//     let name = format!("{} {}", first_name, last_name);
//     return &name
// }

// ! MUTABLE REFERENCE
/**
 * ? dalam satu lifecycle kita hanya bisa punya 1 mutable reference,
 * ? dan juga dalam satu lifecycle jika pakai reference maka semuanya harus reference, tidak boleh ada 1 pun mut referecene
 * ? begitu juga jika ada 1 mut ref maka tidak boleh ada imutable ref lainya
 */
fn change_string_value(value: &mut String) {
    value.push_str(" updated");
}

fn change_int32_value(value: &mut i32) {
    *value = 100 // kita harus men derefence value agar bisa di ubah
}

#[test]
fn test_mutable_reference() {
    let mut name = String::from("ilham");
    let mut age = 26;

    change_string_value(&mut name);
    println!("name is {}", name);

    change_int32_value(&mut age);
    println!("age is {}", age);

    // ! ga bisa di lakukan
    // let test = &mut name; ? dalam satu waktu kita hanya bisa punya 1 mutable reference,
    // let test2 = &mut name; // ini ga bisa di lakukan (karna sudah di pinjak ama variable test), karna dalam 1 lifecycle hanya bisa punya 1 mutable reference
    // change_string_value(test);
    // change_string_value(test2);

    change_string_value(&mut name);
    change_string_value(&mut name);
    change_string_value(&mut name);
    change_string_value(&mut name);
    
}


// ! REFERENCE & BORROWING
/**
 * ? dengan reference ini berarti kita mengirim hanya reference, 
 * ? bukan valueyang berarti tidak akan ada perpindahan owner
 * ? konsep ini mirip golang reference, dimana reference itu bukan value melainkan hanya pointer address
 * ? akan tetapi di rust. kita tidak bisa mengubah value dari reference walau ownernya imutable
 * ? di rust konsep ini namanya borrowing, jd meminjam, hanya meminjam tidak bisa mengubah
 */

fn full_name_with_reference(first_name: &String, last_name: &String) -> String {
    // walau firstname (mutable) dia tidak akan bisa di ubah disini karna first_name disini hanya borrowing
    // first_name.push_str("updated");
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name_with_reference() {
   let mut first_name = String::from("ilham");
   let last_name = String::from("gilang");

    // yang di kirim hanya reference nya saja
   let full_name = full_name_with_reference(&first_name, &last_name);

   println!("first name is {}", first_name); // * kita masih bisa akses ini karna tidak pindah owner ke aprameter. 
   println!("last name is {}", last_name); // * kita masih bisa akses ini karna tidak pindah owner ke aprameter. 
   println!("full name is {}", full_name);
}



// !untuk mengakali kehilangan owner seperti test sebelumnya, kita bisa pake cara ini

 fn full_name2(first_name: String, last_name: String) -> (String, String, String) {
     let full_name =format!("{} {}", first_name, last_name);
     (first_name, last_name, full_name)
 }

 #[test]
 fn test_full_name2() {
    let first_name = String::from("ilham");
    let last_name = String::from("gilang");

    let (first_name, last_name,full_name) = full_name2(first_name, last_name);
    println!("first name is {}", first_name);
    println!("last name is {}", last_name);
    println!("full name is {}", full_name);
 }

 // !function return value ownership movement
/**
 * ? ketika fn itu mereturn value, maka ownershipnya akan pindah ke variable yang menjalankan fn,
 * ? misal: let name = function(); maka akan pindah ownershipnya ke name
 */

 fn full_name(first_name: String, last_name: String) -> String {
     format!("{} {}", first_name, last_name)
 }

 #[test]
 fn test_full_name() {
     let first_name = String::from("ilham");
     let last_name = String::from("gilang");

     let full_name = full_name(first_name, last_name);
    //  ! ownership dari first_name dan last_name akan hilang setelah function full_name selesai
    //  println!("first name is {}", first_name);
    //  println!("last name is {}", last_name);

     println!("full name is {}", full_name);
 }

// !function ownership movement
/**
 * ?masih ingat ketika apapun yang di taro di heap, maka ketika itu di assign ke variable baru
 * ?dia akan pindah ownershipnya. bahkan jika kita hanya menjadikanya parameter/argument di function
 */
fn print_number(number:i32) { // ? ini tidak akan memindahkan ownershipnya
    println!("number is {}", number);
}

fn hi(name: String){ // ! karna name itu String(heap), maka ownershipnya akan pindah ke parameter
    println!("hello {}", name);
}

#[test]
fn test_function_ownership() {
    let a = 10;
    let name = String::from("ilham");
    print_number(a);
    println!("number still available {}", a); //? ini masih ada karna ini di simpan di stack karna fix sized

    hi(name); // ? kalo mao tidak hilang ya gunakan clone seperti di test sebelumnya
    // ! ini akan sedikit membingungkan karna kita tidak bisa print name disini. jd harus ingat apapun itu jika di heap, maka pindah ownership
    // ! jadi ketika function hi selesai, name akan di hapus (otomatis karna rust tidak menggunakan GC/manual memory management)
    // println!("name still available {}", name); //! ini akan error karna name sudah di miliki argument function hi
}


// ########################################



// ########################################
// * CLONE

#[test]
fn clone() {
    let a = String::from("ilham");
    let b = a.clone(); // ? dengan menggunakan clone, kita masih bisa akses a karna ownernya tidak pindah
    println!("a is {}, b is {}", a, b);
}



/*
 OWNERSHIP MOVEMENT 
*/
/*apapun yang di simpan di heap, dia akan pindah ownershipnya. 
karna heap tidak seperti stack yang mengcopy value.
sebaliknya heap akan merefence dan memindahkan ownershipnya (ini mirip golang reference, hanya aja di golang mah ga ilang)
*/
#[test]
fn owenership_movement() {
    let name1 = String::from("ilham");
    println!("name1 is {}", name1);
    let name2 = name1;
    println!("name2 is {}", name2);
    // karna name1 sudah di miliki name2, maka name1 akan di hapus dari memory, menjadikanya tidak bisa di akses
    // println!("name1 {}", name1); //! borrow of moved value: `name1
}

/*DATA COPY */
#[test]
fn data_copy() {
    let mut a = 10;
    let mut b = a; // di memory, sebenernya b itu variable baru yang memiliki value copy-an dari variable a.
    println!("a is {}, b is {}", a, b);

    // disini membuktikan kalo b=a itu tidak reference, alias copy, jadi ketika b di ubah, a tidak akan berubah
    // meskipun a itu mutable
    b = 20; 
    println!("a is {}, b is {}", a, b);
    
    a = 100; // disini kita liat b tidak terganggu valuenya karna value b itu hasil dari value a saat belum di ubah.
    println!("a is {}, b is {}", a, b);
}

/*OWNERSHIP RULES
1. setiap value harus mempunyai 1 owner dalam satu waktu, bisa beda owner tp harus cuma punya 1 owner
2. 
jika owner sudah keluar dari scope maka owner akan di hapus dan juga valuenya.
*/
#[test]
fn ownership_rules() {
    let a = 10;
    {
        let b= 20;
        println!("a is {}, b is {}", a, b);
    } // b akan di hapus jika block ini sudah selesai. 

    // jadi kita tidak bisa print b disini, karna b sudah di hapus dari memory dan juga bukan scope nya sih wkwkw
    // println!("b is {}", b); // cannot find value `b` in this scope

    println!("a is {}", a);    
}
// ########################################


// ########################################
/*
STRING SLICE (&str) di rust itu tidak bisa diubah dan fix size (ditaro di stack), jika di ubah maka dia hanya membuat value baru
jadi data yang lama masih tetep ada di memory. konsep ini juga mempengaruhi apapun yang fix size seperti i32,f64,array dan lainya yang fix size
*/
#[test]
fn test_string(){
    let name = "gilang";
    println!("hello {}", name);

    let mut mutable_name: &str = "gilang"; // ini akan tetep ada di memory
    println!("hello {}", mutable_name);
    mutable_name = "pamungkas"; // mengubah isi dari mutable_name, jadi dia hanya membuat value baru, value lama masih tetep ada di memory
    println!("hello new {}", mutable_name);
}
// ########################################


// ########################################
/*  
karna rust tidak menggunakna GC/manual memory management
jadi rust itu LIFO(last in first out) untuk mengatur memory management.
rust akan menghapus jika function itu sudah selsai.
*/
// heap and stack
#[test]
fn stack_heap() { // ini akan di hapus terakhir kali.
    function_a(); //ini akan di hapus yang ke dua
    function_b(); // ini akan di hapus pertama kali, jd semua variable akan di hapus dari memory
}

fn function_a() {
    let a = 10;// ini akan berada di stack karna ukuranya pasti
    let b = String::from("gilang"); // ini akan berada di heap karna ukuranya tidak pasti
    println!("a is {}, b is {}", a, b); 
}
fn function_b() {
    let a = 20;// ini akan berada di heap karna ukuranya tidak pasti
    let b = String::from("pamungkas"); // ini akan berada di heap karna ukuranya tidak pasti
    println!("a is {}, b is {}", a, b); 
}
// ########################################

// ########################################
#[test]
fn array(){
    let arr: [i32; 5] = [1,2,3,4,5]; //array sudah harus di tentukan panjang dan tipe datanya, mirip golang
    println!("array is {:#?}, array 0 is {}", arr, arr[0]);

    let mut arr_mut = [1,2,3,4,5];
    arr_mut[0] = 10;
    println!("array is {:#?}, array 0 is {}", arr_mut, arr_mut[0]);
}

#[test]
fn two_dimensional_array(){
    let arr: [[i32; 3]; 3] = [[1,2,3],[4,5,6],[7,8,9]];
    println!("array is {:#?}, array 0 0 is {:?}", arr, arr[0][0]);
}
// ########################################

// ########################################
// karna ini tidak mereturn apapun ini dianggap sebagai unit function dan dia mereturn tuple kosong
fn unit(){ 
    println!("Hello!")
}

#[test]
fn test_unit(){
    println!("unit adalah sebuah tuple kosong {:?}", unit());
}
// ########################################

#[test]
fn tuples(){
    let tuple:(i32,f64,bool) = (1,2.0,true);
    println!("tuple is {:#?}", tuple);

    let (x,y,_) = tuple;
    println!("destructuring tuple {},{}",x,y);

    let mut tuple2: (i32, f64, bool) = (1,2.0,true);
    tuple2.2 = false;
    println!("tuple2 is {:#?}", tuple2);

}

#[test]
fn test_variable(){
    let name: &str = "ilham gilang";
    println!("hello {}", name);

    let mut mutable_name = "gilang"; // tambahkan mut sebelum nama variable jika ingin bisa di mutable
    println!("hello {}", mutable_name);

    mutable_name = "pamungkas"; // variable mutable bisa diubah tp dengan tipe data yang sama
    println!("hello {}", mutable_name);

    // di rust kita bisa timpa variable (shadowing), tp kita tidak bisa dapetin variable yang lama
    let name: isize = 10;
    println!("new name variable is {}", name);
}


#[test]
fn hello_test(){
    print!("hello world")
}