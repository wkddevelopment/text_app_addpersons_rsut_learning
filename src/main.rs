/* 
?                   Projektbeschreibung

Erstelle mit Hilfe einer Hashtabelle und Vektoren eine Textschnittstelle, die es einem Benutzer ermöglicht, 
Mitarbeiternamen zu einer Abteilung in einem Unternehmen hinzuzufügen. Zum Beispiel „Sally zur Technik hinzufügen“
oder „Amir zum Vertrieb hinzufügen“. Lass den Benutzer dann eine alphabetisch sortierte Liste aller Personen 
in einer Abteilung oder aller Personen in der Firma nach Abteilung ausgeben.

*/

//? Einbindung der Bibliothek zum Einlesen von Benutzereingaben:
use text_io::read;

//? Deklaration des Person-Datentyps:
#[derive(Debug)]
struct Person {
    vorname: String,
    nachname: String,
    abteilung: String
}

fn main() {

    //? Anlegen eines Vectors zum halten der neu angelegten Personen:
    let mut datenbank: Vec<Person> = Vec::new();
    
    //? Erste Benutzerausgabe:
    println!("Willkomen bei der API-Schnittstelle von 'Fab 4' zum hinzufügen von Personal zur Unternehmensdatenbank.");
    println!("Sie können das Programm mit dem bBefahl 'q'+ Enter jederzeit abbrechen.");

    //? Start des Programm-Loops:
    loop {

            //? daten holen und in variablen ablegen:
            println!("Bitte sag mir den Vornamen:"); // prompt info nachricht
            let vorname: String = read!("{}\n"); // Benutzereingabe lesen
            if vorname.to_string() == "q"{ // catch Programmabbruch
                break
            }
            println!("Bitte sag mir den Nachname:");
            let nachname: String = read!("{}\n");
            if nachname.to_string() == "q"{
                break
            }
            println!("Bitte sag mir die Abteilung:");
            let abteilung: String = read!("{}\n");
            if abteilung.to_string() == "q"{
                break
            }           

            //? neue Person mit Benutzerdaten anlegen:
            let neue_person: Person = { Person {
                vorname,
                nachname,
                abteilung,
                }
            };

            //?  in "Datenbank" (Vector) schreiben:
            datenbank.push(neue_person);

        //? Benutzer fragen ob aus Loop aussteigen, Datenbank auslesen oder weitere Person eingeben:
        println!("Wähle: '+' zur Eingabe einer weiteren Person, 'v' für eine Datenbankliste nach Vornamen, 'n' für nach Nachname, 'a' für nach Abteilung und 'z' nach dem Zeitpunkt der Eingabe sortiert. Mit 'q' beendest du das Programm. Bitte Bestätige deine Eingabe mit Enter!");
        let entscheidung: String = read!("{}\n");        
        
        //? Benutzerentscheidung interpretieren und Ergebnis ausgeben:
        match entscheidung.as_str() { // matching der Benutzereingabe... 
            "+" => continue, // ...Loop startet erneut von Beginn...
            "v" => {
                datenbank.sort_by(|person1, person2| person1.vorname.cmp(&person2.vorname)); // ...Datenbank wird mit sort_by() sortiert. Dazu werden jeweils 2 Personen im Vector mit der cmp() auf Grundlage eines ihrer Felder verglichen, hier mit dem Feld vorname. Mehr zur Methode sort: https://doc.rust-lang.org/std/vec/struct.Vec.html?search=sort#method.sort_by
                println!("{:#?}", datenbank);
                break
            },
            "n" => {
                datenbank.sort_by(|person1, person2| person1.nachname.cmp(&person2.nachname)); // wie oben, aber mit Feld nachname...
                println!("{:#?}", datenbank);
                break
            },
            "a" => {
                datenbank.sort_by(|person1, person2| person1.abteilung.cmp(&person2.abteilung)); // wie oben, aber it Feld abteilung...
                println!("{:#?}", datenbank);
                break
            },
            "z" => {println!("{:#?}", datenbank)}, // Einfache Ausgabe des Vectors Datenbank so wie er vorliegt... 
            "q" => break, // Beenden des Loops und somit Sprung zur quit Ausgabe
            &_ => { // catch für ungültige Eingabe d.h. für oben nicht definierte Benutzereingaben. Nach den Nachrichten an den Benutzer startet der Loop neu mit Personen anlegen
                println!("Keine gültige Eingabe.");
                println!("Sie können eine neue Person eingeben:");
                continue}
        }
    }
        //? quit-Ausgabe:
        println!("Danke für die Benutzung, hoffentlich war es hilfreich.");
        //? Programm ist hier zu Ende.
        
}
