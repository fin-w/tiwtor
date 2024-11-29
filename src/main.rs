use lecsicon_db::definitions::LecsiconEntry;
use std::error::Error;
use std::path::PathBuf;
use std::rc::Rc;
use std::{env, process};

fn handle_query(
    query: String,
    mut recent_searches: Vec<String>,
    db_connection: &mut diesel::SqliteConnection,
) -> Vec<String> {
    if !recent_searches.contains(&query) {
        recent_searches.push(query.clone());
    } else {
        recent_searches.retain(|s| *s != query);
        recent_searches.push(query.clone());
    }
    if let Some(entries) = lecsicon_db::search(query.as_str(), db_connection) {
        for e in entries {
            pretty_print(e.clone(), lecsicon_db::related(e, db_connection));
        }
    }
    recent_searches
}

fn pretty_print(entry: LecsiconEntry, related_entries: Option<Vec<LecsiconEntry>>) {
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}",
        &entry.word,
        match &entry.id - 100 {
            0 => "\x1b[90m⁰\x1b[0m",
            1 => "\x1b[90m¹\x1b[0m",
            2 => "\x1b[90m²\x1b[0m",
            3 => "\x1b[90m³\x1b[0m",
            4 => "\x1b[90m⁴\x1b[0m",
            5 => "\x1b[90m⁵\x1b[0m",
            6 => "\x1b[90m⁶\x1b[0m",
            7 => "\x1b[90m⁷\x1b[0m",
            8 => "\x1b[90m⁸\x1b[0m",
            9 => "\x1b[90m⁹\x1b[0m",
            10 => "\x1b[90m¹⁰\x1b[0m",
            11 => "\x1b[90m¹¹\x1b[0m",
            12 => "\x1b[90m¹²\x1b[0m",
            13 => "\x1b[90m¹³\x1b[0m",
            14 => "\x1b[90m¹⁴\x1b[0m",
            15 => "\x1b[90m¹⁵\x1b[0m",
            _ => "\x1b[31mID_OUT_OF_RANGE\x1b[0m",
        },
        if let Some(lemma) = &entry.lemma {
            if *lemma != entry.word {
                format!(" ← {}", &entry.lemma.unwrap())
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        },
        if let Some(tag) = &entry.tag {
            match tag {
                lecsicon_db::definitions::Tag::Adjective => " \x1b[97m• \x1b[35mansoddair\x1b[0m",
                lecsicon_db::definitions::Tag::Adposition => " \x1b[97m• \x1b[92marddodiad\x1b[0m",
                lecsicon_db::definitions::Tag::Adverb => " \x1b[97m• \x1b[35madferf\x1b[0m",
                lecsicon_db::definitions::Tag::Conjugation => {
                    " \x1b[97m• \x1b[92mcysylltair\x1b[0m"
                }
                lecsicon_db::definitions::Tag::Determiner => {
                    " \x1b[97m• \x1b[92mansoddair dangosol\x1b[0m"
                }
                lecsicon_db::definitions::Tag::Interjection => " \x1b[97m• \x1b[92mebychiad\x1b[0m",
                lecsicon_db::definitions::Tag::Noun => " \x1b[97m• \x1b[92menw\x1b[0m",
                lecsicon_db::definitions::Tag::Number => " \x1b[97m• \x1b[33mrhif\x1b[0m",
                lecsicon_db::definitions::Tag::Particle => " \x1b[97m• \x1b[92mgeiryn\x1b[0m",
                lecsicon_db::definitions::Tag::Pronoun => " \x1b[97m• \x1b[92mrhagenw\x1b[0m",
                lecsicon_db::definitions::Tag::ProperNoun => " \x1b[97m• \x1b[92menw priod\x1b[0m",
                lecsicon_db::definitions::Tag::Verb => {
                    if entry.verb_form.is_some() {
                        " \x1b[97m• \x1b[91mberfenw\x1b[0m"
                    } else {
                        " \x1b[97m• \x1b[31mberf\x1b[0m"
                    }
                }
            }
        } else {
            ""
        },
        match &entry.gender {
            None => "",
            Some(g) => match g {
                lecsicon_db::definitions::Gender::Masculine => "\x1b[94m gwrywaidd\x1b[0m",
                lecsicon_db::definitions::Gender::Feminine => "\x1b[95m benywaidd\x1b[0m",
                lecsicon_db::definitions::Gender::FeminineMasculine =>
                    "\x1b[95m benywaidd\x1b[0m neu\x1b[94m wrywaidd\x1b[0m",
            },
        },
        match &entry.mood {
            None => "",
            Some(mood) => match mood {
                lecsicon_db::definitions::Mood::Indicative =>
                    " \x1b[97m•\x1b[0m modd\x1b[93m mynegol\x1b[0m",
                lecsicon_db::definitions::Mood::Subjunctive =>
                    " \x1b[97m•\x1b[0m modd\x1b[93m dibynnol\x1b[0m",
                lecsicon_db::definitions::Mood::Imperative =>
                    " \x1b[97m•\x1b[0m modd\x1b[93m gorchmynnol\x1b[0m",
            },
        },
        match &entry.degree {
            None => "",
            Some(degree) => match degree {
                lecsicon_db::definitions::Degree::Positive =>
                    " \x1b[97m•\x1b[0m gradd\x1b[93m cadarnhaol\x1b[0m",
                lecsicon_db::definitions::Degree::Comparative =>
                    " \x1b[97m•\x1b[0m gradd\x1b[93m cymharol\x1b[0m",
                lecsicon_db::definitions::Degree::Equative =>
                    " \x1b[97m•\x1b[0m gradd\x1b[93m cyfartal\x1b[0m",
                lecsicon_db::definitions::Degree::Superlative =>
                    " \x1b[97m•\x1b[0m gradd\x1b[93m eithaf\x1b[0m",
            },
        },
        match &entry.tense {
            None => "",
            Some(tense) => match tense {
                lecsicon_db::definitions::Tense::Future =>
                    " \x1b[97m•\x1b[0m amser\x1b[93m dyfodol\x1b[0m",
                lecsicon_db::definitions::Tense::Present =>
                    " \x1b[97m•\x1b[0m amser\x1b[93m presennol\x1b[0m",
                lecsicon_db::definitions::Tense::Past =>
                    " \x1b[97m•\x1b[0m amser\x1b[93m gorffennol\x1b[0m",
                lecsicon_db::definitions::Tense::Imperfect =>
                    " \x1b[97m•\x1b[0m amser\x1b[93m amhenodol\x1b[0m",
                lecsicon_db::definitions::Tense::Plusquamperfekt =>
                    " \x1b[97m•\x1b[0m amser\x1b[93m gorberffaith\x1b[0m",
            },
        },
        match &entry.style {
            None => "",
            Some(style) => match style {
                lecsicon_db::definitions::Style::Archaic =>
                    " \x1b[97m•\x1b[93m hen\x1b[0m dull\x1b[0m",
                lecsicon_db::definitions::Style::Formal =>
                    " \x1b[97m•\x1b[0m  dull\x1b[93m ffurfiol\x1b[0m",
                lecsicon_db::definitions::Style::Colloquial =>
                    " \x1b[97m•\x1b[0m  dull\x1b[93m anffurfiol\x1b[0m",
            },
        },
        match &entry.number {
            None => "",
            Some(number) => match number {
                lecsicon_db::definitions::Number::Singular =>
                    " \x1b[97m•\x1b[0m ffurf\x1b[93m unigol\x1b[0m",
                lecsicon_db::definitions::Number::Plural =>
                    " \x1b[97m•\x1b[0m ffurf\x1b[93m lluosog\x1b[0m",
                lecsicon_db::definitions::Number::Collective =>
                    " \x1b[97m•\x1b[0m ffurf\x1b[93m torfol\x1b[0m",
            },
        },
        if entry.position.is_some() {
            " \x1b[97m•\x1b[93m rhagflaenu\x1b[0m enwau\x1b[0m"
        } else {
            ""
        },
        // last of all include plurals
        if let Some(r_e) = related_entries.clone() {
            let mut related_entries = r_e.clone();
            related_entries.retain(|e| {
                e.mutation.is_none()
                    && e.number == Some(lecsicon_db::definitions::Number::Plural)
                    && e.tag != Some(lecsicon_db::definitions::Tag::Verb)
            });
            let mut plurals_string: String;
            if !related_entries.is_empty() {
                plurals_string = " \x1b[97m•\x1b[93m lluosog: \x1b[0m".to_string();
                for e in related_entries {
                    plurals_string.push_str(e.word.as_str());
                    plurals_string.push_str(", ");
                }
                plurals_string = plurals_string[..plurals_string.len() - 2].to_string();
                plurals_string
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    );

    // Print any related words, e.g. mutations or verb forms.
    if let Some(r_e) = &related_entries {
        let mut related_entries = r_e.clone();
        related_entries.retain(|e| {
            !(e.number == Some(lecsicon_db::definitions::Number::Plural)
                && e.mutation.is_none()
                && e.tag != Some(lecsicon_db::definitions::Tag::Verb))
        });
        let mut related_words = related_entries
            .iter()
            .map(|e| e.word.as_str())
            .collect::<Vec<&str>>();
        related_words.sort_unstable();
        related_words.dedup();
        for r_w in &related_words {
            print!(" {}", r_w);
        }
        if !related_words.is_empty() {
            println!();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let db_file_path = PathBuf::from(&env::var("DATABASE_PATH")?);
    let csv_file_path = PathBuf::from(&env::var("CSV_PATH")?);
    let mut db_connection = lecsicon_db::connection(&db_file_path)?;
    let args: Vec<String> = env::args().collect();
    if !db_file_path.exists() && !csv_file_path.exists() {
        println!(
            "Ni chanfuwyd y ffeil CSV: {}",
            csv_file_path.to_string_lossy()
        );
        process::exit(2);
    } else if !db_file_path.exists() && csv_file_path.exists() {
        println!("Cynhyrchu'r data, arhoswch...");
        lecsicon_db::save_csv_as_sqlite_db(&csv_file_path, &db_file_path);
    }
    match args.len() {
        1 => {
            let mut recent_searches: Vec<String> = vec![];
            loop {
                if let Some(prompt) =
                    lecsicon_db::text_prompt(&db_file_path, recent_searches.clone())
                {
                    match prompt.prompt() {
                        Ok(query) => {
                            recent_searches = handle_query(
                                query,
                                recent_searches.clone(),
                                Rc::get_mut(&mut db_connection).unwrap(),
                            );
                        }
                        Err(_) => {
                            println!();
                            process::exit(1);
                        }
                    }
                }
            }
        }
        _ => {
            println!("Could not process command line args.");
        }
    }
    Ok(())
}
