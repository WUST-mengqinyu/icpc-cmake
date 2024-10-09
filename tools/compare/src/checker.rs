use std::str;

pub trait Checker {
    fn check<S: AsRef<[u8]>>(&self, a: S, b: S) -> super::CompareResult {
        let a = str::from_utf8(a.as_ref());
        let b = str::from_utf8(b.as_ref());
        if a.is_err() || b.is_err() {
            return crate::CompareResult::RE(format!(
                "some output not utf8: {:?} {:?}",
                a.err(),
                b.err()
            ));
        };
        self.check_inner(a.unwrap(), b.unwrap())
    }
    fn check_inner(&self, a: &str, b: &str) -> super::CompareResult;
}

pub struct WCmp;

impl Checker for WCmp {
    fn check_inner(&self, a: &str, b: &str) -> crate::CompareResult {
        let a_spilt = a.split_whitespace().collect::<Vec<_>>();
        let b_spilt = b.split_whitespace().collect::<Vec<_>>();
        if a_spilt.len() != b_spilt.len() {
            return crate::CompareResult::WA(
                "words counts not equal".to_string(),
                (a_spilt.len().to_string(), b_spilt.len().to_string()),
            );
        }
        for (i, (a, b)) in a_spilt.into_iter().zip(b_spilt.into_iter()).enumerate() {
            if !a.eq(b) {
                return crate::CompareResult::WA(
                    format!("words not equal in {i}"),
                    (a.to_owned(), b.to_owned()),
                );
            }
        }
        crate::CompareResult::AC
    }
}

pub struct DynamicLibChecker(std::path::PathBuf);

impl Checker for DynamicLibChecker {
    fn check_inner(&self, a: &str, b: &str) -> crate::CompareResult {
        todo!()
    }
}