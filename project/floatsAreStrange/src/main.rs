// ----------------------------------------------- //
// Introduzione ai numeri floating point (virgola mobile) in Rust
//
// In Rust esistono due principali tipi di float:
// - `f32`: numero in virgola mobile a 32 bit (precisione singola)
// - `f64`: numero in virgola mobile a 64 bit (precisione doppia)
//
// Per default, Rust usa `f64` per i numeri decimali.
// Questi tipi sono utili per rappresentare numeri reali,
// ma hanno limitazioni nella precisione binaria.
// ----------------------------------------------- //

fn main() {
    let pi = 3.1415; // Questo esce in automatico un float a 64bit

    let pi2: f32 = 3.1415927; // Qui forziamo esplicitamente un f32 (32-bit float)
    let decimal: f64 = 2.7718281828459045; // Dichiarazione esplicita di f64 (doppia precisione)

    println!("pi= {pi2}");
    println!("decimal= {}", decimal);

    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let sum: f64 = a + b;
    println!("sum= {}", sum); // Stamperà un numero abbastanza particolare: sum= 0.3000000000000000000004;
    // Anche se matematicamente 0.1 + 0.2 = 0.3, il risultato non è esattamente 0.3

    // se dovessimo scrivere:
    println!("{}", sum == 0.3); // uscirà False perché il valore calcolato non è esattamente uguale a 0.3
}
