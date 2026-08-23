pub fn est_majeur(age: u32) -> bool {
    if age > 18 {
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
}
