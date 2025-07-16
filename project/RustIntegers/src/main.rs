fn main() {
    println!("// ---------------------------------------------- //");
    let n1: i8 = 10; // Integer (Can be POSITIVE or NEGATIVE) -- -128 to 127
    let n2: u8 = 200; // Unsigned Integer (ALWAYS BE POSITIVE) -- 0 to 255

    println!("Il valore n1 è pari a: {} e la sua dimensione massima e minima sono corrispetivamente: {} e {}", n1, i8::MAX, i8::MIN);
    println!("Il valore n1 è pari a: {} e la sua dimensione massima e minima sono corrispetivamente: {} e {}", n2, u8::MAX, u8::MIN);

    println!("// ---------------------------------------------- //");
    /*
    Se dovessimo scrivere:
    let n: i8 = 1000; --> Verrà restituito un errore perché con un intero a 8 bit possiamo arrivare fino a 127
    */
    let n = 1000; // E' come scrivere in questo caso let n: u32 = 1000;

    // ----------------------------------------------------------------------------- //

    /* Esiste un altro tipo di lunghezza che è quella "ARCH" ed è due tipi:
        - Per il signed abbiamo 'ISIZE':
        - Per il unsigned abbiamo 'USIZE':
    Questi due dipendono dall'architettura che il nostro calcolatore possiede

    */

    let n3: isize = 1000;
    println!("Il valore n è pari a: {}", n3);

    // oppure //
    
    println!("Grandezza massima che contiene l'architettura del nostro PC è {}", isize::MAX);
    println!("Grandezza minima che contiene l'architettura del nostro PC è {}", isize::MIN);

    println!("// ---------------------------------------------- //");

    let mut x: u8 = 255;
    // NOTA BENE => x+=10; LA NOSTRA VARIABILE X può essere un valore tra 0 e 255
        // println!("x={}", x);  Darà errore perché stiamo andando oltre la dimensione massima che può contenere la variabile x in 8 bit

        let trillion: i64 = 1000000000000; // Leggere il numero così è molto complicato
        let formatted: i64 = 1_000_000_000_000; // Rust, fortunatamente ignora gli '_' e prende il numero completo lo stesso

        println!("Il numero non formattato {}", trillion);
        println!("Il numero formattato {}", formatted);

}

