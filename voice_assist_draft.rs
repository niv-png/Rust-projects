use std::process::Command;
use std::{thread, time};
use wikipedia::{Wikipedia, PageSummary};
use serde_json::Value;
use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = pyttsx3::init()?;
    let voices = engine.property("voices")?;

    engine.set_property("voice", &voices[1].id)?;

    loop {
        let query = take_command()?;
        let query = query.trim().to_lowercase();

        if query.contains("wikipedia") {
            let search_term = query.replace("wikipedia", "").trim();
            let result = get_wikipedia_summary(search_term)?;

            speak(&format!("According to wikipedia, {}", result));
        } else if query.contains("are you") {
            speak("I am Amigo developed by Jaspreet Singh");
        } else if query.contains("open youtube") {
            open_website("https://www.youtube.com/");
        } else if query.contains("open google") {
            open_website("https://www.google.com/");
        } else if query.contains("open github") {
            open_website("https://github.com/");
        } else if query.contains("open stackoverflow") {
            open_website("https://stackoverflow.com/");
        } else query.contains("open spotify") {
            open_website("https://www.spotify.com/");
        }
    }
 }draft
