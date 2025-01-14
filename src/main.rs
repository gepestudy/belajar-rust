
/*TOLONG BACA DARI BAWAH KE ATAS. */

fn main() {
    println!("Hello, world!");
}

// ########################################

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