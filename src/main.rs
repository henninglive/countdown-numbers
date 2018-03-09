
#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

type Expr = (Operator, Box<Term>, Box<Term>);

#[derive(Debug, Clone)]
struct Term {
    expression: Option<Expr>,
    value: usize,
}

#[derive(Debug)]
struct Solver {
    remaining: Vec<Box<Term>>,
    solutions: Vec<Box<Term>>,
    target: usize,
    counter: usize,
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Operator::*;
        match self.expression {
            Some((ref op, ref a, ref b)) => {
                match *op {
                    Addition => write!(f, "({} + {})", a, b),
                    Subtraction => write!(f, "({} - {})", a, b),
                    Multiplication => write!(f, "({} * {})", a, b),
                    Division => write!(f, "({} / {})", a, b),
                }
            },
            None => write!(f, "{}", self.value),
        }
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        use Operator::*;

        if self.value != other.value {
            return false;
        }

        match (&self.expression, &other.expression) {
            (&Some((ref op1, ref a1, ref b1)),
             &Some((ref op2, ref a2, ref b2))) =>
            {
                match (op1, op2) {
                    (&Addition, &Addition) => (),
                    (&Subtraction, &Subtraction) => (),
                    (&Multiplication, &Multiplication) => (),
                    (&Division, &Division) => (),
                    _ => return false,
                }

                a1.eq(a2) && b1.eq(b2)
            },
            (&None, &None) => true,
            _ => false,
        }
    }
}

impl Solver {
    fn new(numbers: &[usize], target: usize) -> Solver {
        let mut remaining = numbers.iter()
            .map(|i| Box::new(Term{
                expression: None,
                value: *i,
            })).collect::<Vec<_>>();

        remaining.sort_by(|a, b| a.value.cmp(&b.value).reverse());

        Solver {
            remaining: remaining,
            solutions: Vec::new(),
            target: target,
            counter: 0,
        }
    }

    fn try_expr(&mut self, expr: Expr) -> Expr {
        let mut c = Box::new(match expr.0 {
            Operator::Addition => Term {
                value: expr.1.value + expr.2.value,
                expression: Some(expr),
            },
            Operator::Subtraction => {
                // Negative intermediate values are not allowed in countdown 
                // and zero is not a useful term.
                if expr.1.value <= expr.2.value {
                    return expr;
                }
                Term {
                    value: expr.1.value - expr.2.value,
                    expression: Some(expr),
                }
            },
            Operator::Multiplication => Term {
                value: expr.1.value * expr.2.value,
                expression: Some(expr),
            },
            Operator::Division => {
                // Fractions are not allowed in countdown
                if expr.1.value % expr.2.value != 0 {
                    return expr;
                }
                Term {
                    value: expr.1.value / expr.2.value,
                    expression: Some(expr),
                }
            },
        });

        self.counter += 1;
        
        // Test if this is a valid solution
        if c.value == self.target && !self.solutions.contains(&c) {
            self.solutions.push(c.clone());
        }

        if self.remaining.len() > 0 {
            // Find Insert position so self.remaining remains sorted
            let pos = {
                let mut pos = 0;
                let mut iter = self.remaining.iter();
                while let Some(k) = iter.next() {
                    if k.value <= c.value {
                        break;
                    }
                    pos += 1;
                }
                pos
            };

            self.remaining.insert(pos, c);
            self.solve();
            c = self.remaining.remove(pos);
        }
        c.expression.unwrap()
    }

    fn solve(&mut self) {
        for i in 0..self.remaining.len() {
            let mut a = self.remaining.remove(i);
            for j in i..self.remaining.len() {
                let mut expr = (Operator::Addition, a, self.remaining.remove(j));
                expr = self.try_expr(expr);

                expr.0 = Operator::Subtraction;
                expr = self.try_expr(expr);

                expr.0 = Operator::Multiplication;
                expr = self.try_expr(expr);

                expr.0 = Operator::Division;
                expr = self.try_expr(expr);

                self.remaining.insert(j, expr.2);
                a = expr.1;
            }
            self.remaining.insert(i, a);
        }
    }
}

fn main() {
    let (numbers, target) = ([25, 50, 75, 100, 8, 9].to_vec(), 952);

    // convert numbers to string and join together
    let numbers_str = {
        let mut numbers_str = String::new();
        let mut first = true;
        for s in numbers.iter() {
            if first {
                first = false;
            } else {
                numbers_str.push_str(", ");
            }
            numbers_str.push_str(&s.to_string());
        }
        numbers_str
    };

    println!("Starting numbers: [{}], target: {}", numbers_str, target);
    
    let mut solver = Solver::new(&numbers[..], target);

    let start_time = std::time::Instant::now();
    solver.solve();
    let elapsed = start_time.elapsed();

    println!("{} Valid expressions, found {} Solutions in {}.{:09} seconds",
        solver.counter, solver.solutions.len(),
        elapsed.as_secs(), elapsed.subsec_nanos());

    for s in solver.solutions.iter() {
        println!("{} = {}", s, s.value);
    }
}