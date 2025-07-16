fn main() {
    // 1. Variabile immutabile
    let number = 10; // Variabile immutabile: non può essere modificata dopo l'inizializzazione

    println!("Number: {}", number);         // Stampa: Number: 10
    println!("Number: {number}");           // Interpolazione alternativa: Number: 10

    /*
    ERRORE: Questo codice causerebbe un errore di compilazione
    perché 'number' è immutabile:
    
        number = 20;
        println!("Number: {}", number);
    */
    // -------------------------------------------------- //

    // 2. Variabile mutabile
    let mut mutable_number = 20; // `mut` rende la variabile modificabile

    println!("Mutable Number: {}", mutable_number);  // Stampa: 20
    mutable_number = 30;
    println!("Mutable Number: {}", mutable_number);  // Stampa: 30

    // -------------------------------------------------- //

    // 3. Convenzioni di denominazione (Naming Conventions)
    // In Rust si usa lo `snake_case` per le variabili

    let first_name = "Deniel";
    println!("My name is {}", first_name);  // Stampa: My name is Deniel

    // -------------------------------------------------- //

    // 4. Costanti in Rust
    // Le costanti sono sempre immutabili, si usano con `const` e richiedono un tipo esplicito

    const ONE_MINUTE: i32 = 60;
    const ONE_HOUR: i32 = ONE_MINUTE * 60;

    println!("One minute is {} seconds", ONE_MINUTE); // Stampa: 60
    println!("One hour is {} seconds", ONE_HOUR);     // Stampa: 3600

    // Esempio con pi greco
    const PI: f64 = 3.1415;
    println!("Pi is {}", PI);                         // Stampa: 3.1415

    /*
    ATTENZIONE: Non è possibile fare queste operazioni:

        // Errore: cambio di tipo non consentito
        const PI: i32 = 3;

        // Errore: uso scorretto di `let` per costante globale
        let PI: f64 = 3.1415;

    Le costanti devono avere tipo fisso e non possono essere ridefinite con `let`.
    */
}
