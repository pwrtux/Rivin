

#[cfg(test)]


mod tests {
    

    use crate::findyear;


    #[test]
    fn check_findyea_success() {
        assert_eq!(findyear('1'), "2000 or 1970");
        assert_eq!(findyear('Y'), "2029 or 1999");
    }

    #[test]
    fn check_failure(){
        assert_eq!(findyear('Z'), "No year found");
    }

}

