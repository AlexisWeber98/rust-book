fn main() {
    let mut x = 5;
    println!("The value of X: {x}");
    x = 6;

    println!("The value of X: {x}");

    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing

    println!(" ------------------ SHADOWiNG ------------------------- ");
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
     * falla: Rust puede inferir el tipo de dato pero en este caso no puede,porque le falta
     * informacion, por lo que debemosindicarle el tipo de dato
     */

    // ------------------------ NUMERICOS ------------------------//

    println!(" ---------------------- NUMERICOS ---------------------- ");

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

    println!(" ---------------- FLOTANTES -----------------");
    let x = 3.0; // f64
    let y: f32 = 6.1;

    println!("Value of x: {x} \nValue of y: {y}");
    // OPERACIONES

    println!(" ---------------- OPERACIONES -----------------");
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

    println!(" ------------ BOOLEANOS ----------------------");
    let _t = true;
    let _f: bool = false;

    println!("Valeu of t: {_t} \nValue of f: {_f}");
    // ------------- Caracter ------------------- //

    println!(" ------------ CHAR --------------");
    let character: char = 'a';
    let character_two = 'Z';

    println!("Value of character: {character} \nValue of character two: {character_two}");
    // --------------- COMPUESTOS ------------------//

    // tuplas //

    //  las tuplas son inmutables y los valores pueden tener tipos distintos

    println!(" ----------------- TUPLAS -------------------");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of Y is: {y}");
    println!("The value of X is: {x}");
    println!("The value of Z is: {z}");

    let five_hundred = tup.0;

    println!("the value of X is: {five_hundred}");

    let one = tup.2;

    println!("The value of Z is: {one}");

    let _tup_empty = ();

    // arreglos
    /*
     * los arreglos sotn inmutables y sus valores solo pueden tener el mismo tipo
     */

    println!(" ---------------------- ARRAY --------------");
    let _first_array = [1, 2, 3, 4];

    let months = [
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

    for month in &months {
        println!("Month: {}", month);
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let three = [3; 5]; // contiene 5 elemntos cuyo valor sera 3

    for number in &three {
        println!("Number: {}", number);
    }

    let first_element = a[0];

    let second_element = three[1];

    println!("First element of a: {first_element}");

    println!("Second element of trhee: {second_element}");
}
