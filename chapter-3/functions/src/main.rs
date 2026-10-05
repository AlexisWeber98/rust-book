fn main() {
    println!("Hello World!");
    another_function(5);
    print_labeled_measurement(5, 'h');

    expressions();
    let x = five();
    println!("The value of x: {x}");
}

fn another_function(number: i32) {
    println!("The value of number is: {number}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

// --------------------------------- Sentencias y expresiones ----------------------------------- //

/* Sentencia es una instruccion que realiza una accion y no devuelve un valor
*   EJ:
* let y = 6;
*
* (hace algo: declara un valor pero no lo devielve)
*/

// Las exprensiones SI evaluan eun valor:EJ: 5 + 6 (expresion que evalua al valor 11):
fn expressions() {
    let y = {
        let x = 3;
        x + 1 //no tiene punto y coma, por lo tanto es retornable, si tuviera no seria retornable 
    };

    println!("The value of Y is: {y}");
}

// ----------------------------- Funciones con valor de retorno ---------------------- //

fn five() -> i32 {
    5
}

/*
* fn plus_one(x: i32) -> i32 {
+    x + 1;
+}

el Error es que la funcion pide devolver un i32, pero al poner punto y coma a x + 1, la convertimos en sentencia, no se retorna
si le quitamos el punto y coma la volvemos expresion y en ese caso si estaria retornando implisitamente el i32
*/
