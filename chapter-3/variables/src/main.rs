fn main() {
    let mut x = 5;
    println!("The value of X: {x}");
    x = 6;

    println!("The value of X: {x}");

    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let x = x + 1;

    println!("The value of X: {x}");

    {
        let x = x * 2;
        println!("The value of X: {x}");
    }

    println!("The value of X: {x}");

    let spaces = "    ";

    let _spaces = spaces.len();

    /*
     * error: shadowing no es lo mismo que mutabilidad
     * let mut spaces = "    ";
     * spaces = spaces.len();
     */

    // TIPOS DE DATOS

    /*
     *
     * let guess: u32 = "42".parse().expect("Not a number!");
     *
     * falla: Rust puede inferir el tipo de dato per en este caso no puede,porque le falta
     * informacion, por lo que debemosindicarle el tipo de dato
     */

    // ------------------------ NUMERICOS ------------------------//

    //Tamaño                signado       sin signo
    // 8-bit                	i8            	u8
    //16-bit                >i16            >u16
    //32-bit                >i32            >u32
    //64-bit                	i64           	u64
    //128-bit                	i128          	u128
    //Dependiente de la arquitectura	isize	usize

    /*
     * error:
     *let numero: u8 = 20;
     *
     *let _resultado = numero - 35;
     */

    // flotantes

    let _x = 3.0; // f64
    let _y: f32 = 6.1;

    // OPERACIONES

    //adicion
    let sum = 5 + 15;
    println!("adicion: {sum}");

    // sustraccion
    let difference = 83.2 - 2.7;
    println!("sustraccion: {difference}");

    // multiplipacion
    let product = 4 * 45;
    println!("multiplicacion: {product}");

    // divicion
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    println!("divicion: {quotient}");
    println!("divicion truncada: {truncated}");

    // modulo
    let remainder = 23 % 5;
    println!("modulo: {remainder}");

    // ------------------------ BOOLEANO ----------------------------- //

    let _t = true;
    let _f: bool = false;

    // ------------- Caracter ------------------- //

    let _character: char = 'a';
    let _character_two = 'Z';

    // --------------- COMPUESTOS ------------------//

    // tuplas //

    /*
     * las tuplas son inmutablesm y los valores pueden tener tipos distintos
     */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of Y is: {y}");

    let five_hundred = tup.0;

    println!("the value of X is: {five_hundred}");

    let one = tup.2;

    println!("The value of Z is: {one}");

    // arreglos
    /*
     * los arreglos sotn inmutables y sus valores solo pueden tener el mismo tipo
     */

    let _first_array = [1, 2, 3, 4];

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // print!("Montsh: \n {months}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let three = [3; 5]; // contiene 5 elemntos cuyo valor sera 3
    let first_element = a[0];

    let second_element = three[1];

    println!("First element of a: {first_element}");

    println!("Second element of trhee: {second_element}");
}
