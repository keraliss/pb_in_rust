#[derive(Debug, PartialEq)]
struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    fn new(num: i32, prime: i32) -> Result<Self, &'static str> {
        if num >= prime || num < 0 {
            return Err("Num not in field range");
        }
        Ok(FieldElement { num, prime })
    }

    fn representation(&self) -> String {
        format!("{},{}", self.num, self.prime)
    }

    fn eq(&self, other: &Option<FieldElement>) -> bool {
        match other {
            None => false,
            Some(other) => self.num == other.num && self.prime == other.prime,
        }
    }
    fn ne(&self, other: &Option<FieldElement>) -> bool {
        match other {
            None => true,
            Some(other) => self.num != other.num || self.prime != other.prime,
        }
    }

    fn add(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
        if self.prime != other.prime {
            return Err("Cannot add two numbers in different fields");
        }
        let num = (self.num + other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    fn sub(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
        if self.prime != other.prime {
            return Err("Cannot subtract two numbers in different fields");
        }
        let num = (self.num - other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    fn mul(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
        if self.prime != other.prime {
            return Err("Cannot multiply two numbers in different fields");
        }
        let num = (self.num * other.num) % self.prime;
        FieldElement::new(num, self.prime)
    }

    // fn div(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
    //     if self.prime != other.prime {
    //         return Err("Cannot divide two numbers in different fields");
    //     }
    //     let num = (self.num/other.num) % self.prime;
    //     Ok(FieldElement::new(num, self.prime)?)
    // }

    fn pow(&self, exponent: i32) -> FieldElement {
        let n = exponent % (self.prime - 1);
        let num = self.num.pow(n as u32) % self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }

    fn true_div(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
        if self.prime != other.prime {
            return Err("Cannot divide two numbers in different fields");
        }
        let num = (self.num * self.pow(self.prime - 2).num) % self.prime;
        FieldElement::new(num, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ne() {
        let a = FieldElement::new(2, 31).unwrap();
        let b = FieldElement::new(2, 31).unwrap();
        let c = FieldElement::new(15, 31).unwrap();
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a == b);
    }
    #[test]
    fn test_add() {
        let a = FieldElement::new(2, 31).unwrap();
        let b = FieldElement::new(15, 31).unwrap();
        assert_eq!(a.add(&b).unwrap(), FieldElement::new(17, 31).unwrap());
        let a = FieldElement::new(17, 31).unwrap();
        let b = FieldElement::new(21, 31).unwrap();
        assert_eq!(a.add(&b).unwrap(), FieldElement::new(7, 31).unwrap());
    }
    #[test]
    fn test_sub() {
        let a = FieldElement::new(29, 31).unwrap();
        let b = FieldElement::new(4, 31).unwrap();
        assert_eq!(a.sub(&b).unwrap(), FieldElement::new(25, 31).unwrap());
        let a = FieldElement::new(5, 31).unwrap();
        let b = FieldElement::new(30, 31).unwrap();
        assert_eq!(a.sub(&b).unwrap(), FieldElement::new(16, 31).unwrap());
    }
    #[test]
    fn test_mul() {
        let a = FieldElement::new(24, 31).unwrap();
        let b = FieldElement::new(19, 31).unwrap();
        assert_eq!(a.mul(&b).unwrap(), FieldElement::new(22, 31).unwrap());
    }
    #[test]
    fn test_pow() {
        let a = FieldElement::new(17, 31).unwrap();
        assert_eq!(a.pow(3), FieldElement::new(15, 31).unwrap());
        let a = FieldElement::new(5, 31).unwrap();
        let b = FieldElement::new(18, 31).unwrap();
        assert_eq!(a.pow(5).mul(&b), FieldElement::new(16, 31));
    }
    #[test]
    fn tets_div() {
        let a = FieldElement::new(3, 31).unwrap();
        let b = FieldElement::new(24, 31).unwrap();
        assert_eq!(a.true_div(&b).unwrap(), FieldElement::new(4, 31).unwrap());
        let a = FieldElement::new(17, 31).unwrap();
        assert_eq!(a.pow(-3), FieldElement::new(29, 31).unwrap());
        let a = FieldElement::new(4, 31).unwrap();
        let b = FieldElement::new(11, 31).unwrap();
        assert_eq!(a.pow(-4).mul(&b), FieldElement::new(13, 31));
    }
}
