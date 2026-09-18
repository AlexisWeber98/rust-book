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
}
