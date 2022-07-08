struct Employee {
    first: String,
    last: String,
    fixed_salary: f64,
    bonus: f64,
    tax_pct: f64
}

impl Employee {
    fn net_salary(&self) -> f64 {
        self.fixed_salary + self.bonus - (self.fixed_salary + self.bonus) * self.tax_pct / 100.0
    }

    fn associated_function() {//like static methods of OOP language
        println!("This is an associated function for a struct and can be called without an instance");
    }
}

pub fn struct_sample() {
    let emp = Employee {
        first: String::from("Byomkesh"),
        last: String::from("Bakshi"),
        fixed_salary: 1000.0,
        bonus: 100.0,
        tax_pct: 10.0,
    };

    println!("Employee net salary is {}", emp.net_salary());
    Employee::associated_function();
}