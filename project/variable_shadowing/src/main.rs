// In questo file si parla di shadowing delle variabili in Rust. 
// Il concetto di shadowing permette di dichiarare una nuova variabile con lo stesso nome di una precedente, "nascondendo" la vecchia variabile.
// Questo può essere utile per cambiare il tipo di una variabile o per riassegnare un nuovo valore senza usare `mut`.
// Ecco un esempio di come funziona il shadowing in Rust.


fn main() {
    // 1. Esempio di shadowing
    println!("// -------------------------------- //");
    println!("1. Esempio di shadowing in Rust");
    let n = 5; // Dichiarazione di una variabile immutabile

    {
        let n = 15;
        println!("All'interno del primo blocco n vale: {}", n); // Stampa: Inner n: 15
    }

    println!("All'esterno n vale: {}", n); // Stampa: Outer n: 5


    // Se avessimo scritto
    
    {
        println!("All'interno del secondo blocco n vale: {}", n); // Apparirà a video il numero 5 perché la variabile `n` all'interno del blocco non è stata ridefinita.
    }

    // ------------------------------------------------------------------------------------------------------------------------------------------------------------------- //
    // 2. Shadowing con variabili mutabili
    // Adesso mostriamo un altro esempio di shadowing, questa volta cambiando il tipo della variabile:;
    println!("// -------------------------------- //");
    println!("2. Shadowing con variabili mutabili in Rust");
    let spaces = "      "; // Dichiarazione di una variabile immutabile di tipo stringa con 6 spazi
    let lenght_of_spaces = spaces.len(); // Calcolo della lunghezza della stringa

    println!("La lunghezza degli spazi è: {}", lenght_of_spaces); // Stampa: 6

    // però possiamo anche definire lenght_of_spaces come spaces e ricalcoliamo la lunghezza:

    let spaces = spaces.len(); // Qui stiamo "nascondendo" la variabile `spaces` originale, ora `spaces` è un intero
    println!("La lunghezza degli spazi è: {}", spaces); // Stampa: 6

    /* ------------------------------------------------------------------------------------------------------------------------------------------------------------------- //
                                NOTA BENE: Se dovessimo scrivere 

                                ` let mut spaces = "      ";`
                                ` spaces = spaces.len();`

                                otterremmo un errore di tipo perché stiamo cercando di assegnare un intero a una variabile di tipo stringa.
                                Questo perché `spaces` è stata dichiarata come stringa e non può essere cambiata in un intero senza usare il shadowing.
    // ------------------------------------------------------------------------------------------------------------------------------------------------------------------- */
    
}
