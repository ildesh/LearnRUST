/* 
In questo file parleremo di 'Data Types' in Rust, che sono fondamentali per comprendere come gestire le informazioni nel linguaggio.
I tipi di dati in Rust sono divisi in due categorie principali:
      1. tipi scalari: rappresentano un singolo valore e includono:
         - interi
         - numeri in virgola mobile
         - booleani
         - caratteri
      2. tipi composti: possono contenere più valori e includono:
         - tuple
         - array

Inoltre, Rust supporta anche i tipi di dati personalizzati tramite le strutture (struct) e le enumerazioni (enum).

*/


fn main() {
    // Iniziamo con un esempio di input dell'utente e la sua conversione in un tipo numerico
    println!("// -------------------------------- //");
    println!("Esempio di input dell'utente e conversione in un tipo numerico");
    let user_input = "100"; // Simuliamo un input dell'utente come stringa
    let converted_input: u32 = user_input.parse().expect("Input non valido"); // Convertiamo la stringa in un intero con tanto di errore in caso di fallimento

    println!("L'input convertito è: {}", converted_input); // Stampa: L'input convertito è: 100

    // Possiamo, ovviamente dopo la conversione, eseguire delle operazioni matematiche con la variabile 'converted_input'
    let somma = converted_input + 50; // Sommiamo 50 all'input convertito
    let differenza = converted_input - 30; // Sottraiamo 30 all'input convertito
    let prodotto = converted_input * 2; // Moltiplichiamo l'input convert
    let divisione = converted_input / 4; // Dividiamo l'input convertito per 4

    println!(" - Somma: {}", somma); // Stampa: Somma: 150
    println!(" - Differenza: {}", differenza); // Stampa: Differenza: 70
    println!(" - Prodotto: {}", prodotto); // Stampa: Prodotto: 200
    println!(" - Divisione: {}", divisione); // Stampa: Divisione: 25

    // Ora vediamo alcuni tipi di dati scalari in Rust
    println!("// -------------------------------- //");
    println!("Tipi di dati scalari in Rust");
    let number: i8 = 10; // Definiamo una variabile di tipo intero a 8 bit che può contenere valori da -128 a 127
    let pi : f32 = 3.14; // Definiamo una variabile di tipo float a 32 bit per rappresentare il numero pi greco
    let turn_on : bool = true; // Definiamo una variabile booleana che può essere true o false
    let char : char = 'D'; // Definiamo una variabile di tipo carattere, che può contenere un singolo carattere Unicode

    println!(" - Number: {} \n - Pi: {} \n - Turn On: {} \n - Char: {}", number, pi, turn_on, char); // Stampa i valori delle variabili

    // Ora vediamo i tipi composti, come le tuple
    println!("// -------------------------------- //");
    println!("Tuple in Rust");
    let coordinates: (f32, f32) = (10.3, 20.5); // Definiamo una tupla con due numeri in virgola mobile
    println!("Coordinate: ({}, {})", coordinates.0, coordinates.1); // Stampa: Coordinate: (10.3, 20.5)

    // Possiamo anche destrutturare la tupla in variabili separate
    // Questo ci permette di accedere ai singoli elementi della tupla in modo più leggibile
    println!("// -------------------------------- //");
    println!("Destrutturazione di una tupla in Rust");
    let (x, y) = coordinates;
    println!(" - X: {} \n - Y: {}", x, y); // Stampa: - X: 10.3 \n - Y: 20.5

    println!("// -------------------------------- //");
    println!("Array in Rust");
    let people: [&str; 3] = ["Alice", "Bob", "Charlie"]; // Definiamo un array di stringhe
    // Con l'utilizzo di '&str' indichiamo che gli elementi dell'array sono stringhe di tipo 'slice', ovvero riferimenti a stringhe esistenti in memoria
    println!("Persone: {:?}", people); // Stampa: Persone: ["Alice", "Bob", "Charlie"]
    // Con l'utilizzo di {:?} indichiamo che vogliamo stampare l'array in modo leggibile, ovvero nel seguente formato: ["Alice", "Bob", "Charlie"]
    println!("// -------------------------------- //");

}
