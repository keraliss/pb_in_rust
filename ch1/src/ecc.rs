struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    fn new(num: i32, prime: i32) -> Result<Self, &'static str> {
        if num >= prime || num < 0 {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }
        Ok(FieldElement { num, prime })
    }
    fn representation(&self) -> String {
        format!("{},{}", self.num, self.prime)
    }
    fn eq (&self, other: &FieldElement) -> bool {
        if other == None {
            return false
        }
        self.num == other.num && self.prime == other.prime
    }
    fn ne(&self, other: &FieldElement) -> bool {
        if other == None {
            return false
        }
        self.num != other.num || self.prime != other.prime
    }
    fn add(&self, other: &FieldElement) -> Result<FieldElement, &'static str> {
       if self.prime != other.prime {
           return Err("Cannot add two numbers in different fields");
       }
       let num = (self.num + other.num)%self.prime;
       Ok(FieldElement::new(num, self.prime))
    }
    fn sub(&self, other:&FieldElement) -> Result<FieldElement ,&'static str> {
        if self.prime != other.prime {
            return Err("Cannot subtract two numbers in different fields");
        }
        let num = (self.num - other.num)%self.prime;
        Ok(FieldElement::new(num, self.prime))
    }
}