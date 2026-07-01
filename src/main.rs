use std::{fs, process};
use itertools::Itertools;
use std::env::args;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Language {
    name: String,
    cc: f64,
    cv: f64,
    vc: f64,
    vv: f64,
    ratio: f64,
}

struct DetectResult {
    best: String,
    gap: f64,
    second: String,
}
const CC_MIN: f64 = 12.32;
const CC_MAX: f64 = 24.10;
const CV_MIN: f64 = 34.02;
const CV_MAX: f64 = 46.15;
const VC_MIN: f64 = 34.04;
const VC_MAX: f64 = 41.20;
const VV_MIN: f64 = 0.33;
const VV_MAX: f64 = 11.43;
const RATIO_MIN: f64 = 122.08;
const RATIO_MAX: f64 = 164.29;

fn is_vowel(a: char, vowels: &str, consonants: &str) -> bool {
    let lower = a.to_lowercase().next().unwrap_or(a);
    if vowels.chars().contains(&lower) {
        return true
    }
    else if consonants.chars().contains(&lower) {
        return false
    }
    false
}

fn normalize(val: f64, min: f64, max: f64) -> f64 {
    (val - min) / (max - min)
}

fn detect(features: (f64, f64, f64, f64, f64), profiles: &[Language], verbose: bool) -> DetectResult {
    let (cc, cv, vc, vv, ratio) = features;

    let norm_cc = normalize(cc, CC_MIN, CC_MAX);
    let norm_cv = normalize(cv, CV_MIN, CV_MAX);
    let norm_vc = normalize(vc, VC_MIN, VC_MAX);
    let norm_vv = normalize(vv, VV_MIN, VV_MAX);
    let norm_ratio = normalize(ratio, RATIO_MIN, RATIO_MAX);

    let mut langdistances= HashMap::new();

    for profile in profiles {
        let norm_p_cc = normalize(profile.cc, CC_MIN, CC_MAX);
        let norm_p_cv = normalize(profile.cv, CV_MIN, CV_MAX);
        let norm_p_vc = normalize(profile.vc, VC_MIN, VC_MAX);
        let norm_p_vv = normalize(profile.vv, VV_MIN, VV_MAX);
        let norm_p_ratio = normalize(profile.ratio, RATIO_MIN, RATIO_MAX);

        let distance = (
            (norm_cc - norm_p_cc).powi(2)
            + (norm_cv - norm_p_cv).powi(2)
            + (norm_vc - norm_p_vc).powi(2)
            + (norm_vv - norm_p_vv).powi(2)
            + (norm_ratio - norm_p_ratio).powi(2)
        ).sqrt();

        langdistances.insert(profile.name.clone(), distance);

    }
    let mut entries: Vec<(&String, &f64)> = langdistances.iter().collect();
    entries.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    if verbose {
        for (lang, dist) in &entries {
            println!("{}: {}", lang, dist);
        }
    }
    let gap = entries[1].1 - entries[0].1;
    DetectResult {
        best: entries[0].0.clone(),
        gap: gap,
        second: entries[1].0.clone(),
    }
    
}
fn analyze(vowels: &str, consonants: &str, text: String, verbose: bool) -> (f64, f64, f64, f64, f64) {
    let mut cc = 0;
    let mut cv = 0;
    let mut vc = 0;
    let mut vv = 0;
    let mut cons = 0;
    let mut vow = 0;
    for (a1, a2) in text.chars().tuple_windows() {
        if a1.is_alphabetic() && a2.is_alphabetic() {
            match (is_vowel(a1, &vowels, &consonants), is_vowel(a2, &vowels, &consonants)) {
                (false, false) => {cc += 1;}
                (false, true) => {cv += 1;}
                (true, false) => {vc += 1;}
                (true, true) => {vv += 1;}
            }
        }
    }
    for char in text.chars() {
        if char.is_alphabetic() {
            if is_vowel(char, vowels, consonants) {
                vow += 1;
            }
            else {
                cons += 1;
            }
        }
    }
    let total = cc + cv + vc + vv;
    let ccp = (cc as f64 / total as f64) * 100.00;
    let cvp = (cv as f64 / total as f64) * 100.00;
    let vcp = (vc as f64 / total as f64) * 100.00;
    let vvp = (vv as f64 / total as f64) * 100.00;
    let ratio = (cons as f64 / vow as f64) * 100.00;
    if verbose {
        println!("{ccp} {cvp} {vcp} {vvp} {ratio}");
    }
    (ccp, cvp, vcp, vvp, ratio)
}

fn main() {
    let vowels = "aeiouäöüéèêëîïôûùı";
    let consonants = "bcdfghjklmnpqrstvwxyzßñç";
    let args: Vec<String> = args().collect();
    if args.len() > 3 || args.len() < 2 {
        eprintln!("Usage: langdetect <file>");
        process::exit(1);
    }
    let languages = vec![
        Language {
            name: "English".to_string(),
            cc: 23.70,
            cv: 35.67,
            vc: 35.16,
            vv: 5.36,
            ratio: 164.29,
        },
        Language {
            name: "French".to_string(),
            cc: 15.32,
            cv: 39.35,
            vc: 33.04,
            vv: 11.43,
            ratio: 122.08,
        },
        Language {
            name: "German".to_string(),
            cc: 24.10,
            cv: 34.02,
            vc: 35.70,
            vv: 6.40,
            ratio: 164.10,
        },
        Language {
            name: "Turkish".to_string(),
            cc: 12.32,
            cv: 46.15,
            vc: 41.20,
            vv: 0.33,
            ratio: 133.36,
        },
    ];
    let text =  match fs::read_to_string(&args[1]) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            process::exit(1);
        }
    };
    let mut verbose = false;
    if args.len() > 2 && args[2] == "v" {
        verbose = true;
    }
    let language = detect(analyze(vowels, consonants, text, verbose), &languages, verbose);
    if verbose {
        println!("Guess: {}\nGap: {}\nSecond: {}", language.best, language.gap, language.second);
    }
    else {
        println!("{}", language.best);
    }

}
