pub fn est_majeur(age: u32) -> bool {
    if age >= 18 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

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
