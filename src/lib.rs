pub fn est_majeur(age: u32) -> bool {
    if age >= 18 {
        return true;
    }
    false
}

//Écris une fonction moyenne qui prend une slice de notes (&[f64]) et renvoie leur moyenne.
pub fn moyenne(notes: &[f64]) -> f64 {

    let mut moyenne: f64 = 0.0;
    let diviseur = notes.len() as f64;

    for note in notes.iter() {
        moyenne += note;
    }
    moyenne / diviseur
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_moyenne_de_10_et_20_est_de_15() {
        let notes: Vec<f64> = vec![10.0, 20.0];
        assert_eq!(moyenne(&notes), 15.0);
    }

    #[test]
    fn check_age_clairement_majeur() {
        let sevy = est_majeur(24);
        assert!(sevy);
    }

    #[test]
    fn check_age_clairement_mineur() {
        let sevy = est_majeur(9);
        assert!(!sevy)
    }

    #[test]
    fn check_pile_mineur_a_18_ans() {
        let sevy = est_majeur(18);
        assert!(sevy)
    }
}
