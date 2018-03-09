//! This program finds all solutions to a numbers round from the popular
//! British tv show Countdown.
//!
//!
//! ## Rules
//! The rules of the Countdown Numbers Game are as follow:
//!
//! The contestant chooses six numbers from two groups of, 20 small numbers and
//! 4 large numbers. The numbers consist of two each of numbers 1 through 10.
//! The 4 large numbers are 25, 50, 75 and 100. The contestant decides how many
//! large numbers are to be used, from none to all four, the rest will be small
//! numbers.
//!
//! A random three-digit target is generated. The contestants have 30 seconds
//! to work out a sequence of calculations with the numbers whose final result
//! is as close to the target number as possible. They may use only the four
//! basic operations of addition, subtraction, multiplication and division,
//! and do not have to use all six numbers. Fractions are not allowed, and only
//! positive integers may be obtained as a result at any stage of the calculation.
//!
//!
//! ## Algorithm and optimizations
//! The general approach is to recursively combine terms into a binary
//! expression tree while continuously testing if an expression is a valid
//! solution. The rules allow for the flowing optimization:
//!
//! When applying an operator to two terms, we only consider the expression
//! where the terms are from largest to smallest (5 - 3). This a valid since
//! addition and multiplication is commutative, we don’t allow negative
//! values at any intermediate step, we don’t allow fractions.
//!

/// The four basic mathematical operations
#[derive(Debug, Clone, Copy)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

/// Basic mathematical expression with two terms and an operator,
/// forms a binary expression tree.
type Expr = (Operator, Box<Term>, Box<Term>);

/// Mathematical Term
#[derive(Debug, Clone)]
struct Term {
    /// Expression used to calculate this term.
    expression: Option<Expr>,
    /// Integer value of the term
    value: usize,
}


/// Countdown Numbers game solver
#[derive(Debug)]
struct Solver {
    /// Stack of remaining terms
    remaining: Vec<Box<Term>>,
    /// List of solutions found
    solutions: Vec<Box<Term>>,
    /// Target number
    target: usize,
    // Number of expressions evaluated
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
    /// Initiate Solver
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

    /// Test an expression as a solution, then continue combining terms.
    fn try_expr(&mut self, expr: Expr) -> Expr {
        assert!(expr.1.value >= expr.2.value, "terms vector is not sorted");

        // Calculate expression into new term
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

            // Insert new term and continue recursively combining terms.
            // The stack is returned to its original state after the recursive
            // call so we can pop our term, deconstruct it and return
            // the expression when we are done.
            self.remaining.insert(pos, c);
            self.solve();
            c = self.remaining.remove(pos);
        }
        c.expression.unwrap()
    }

    /// Finds all valid expressions resulting in the target number.
    /// Recursively combines two and two terms into a binary expression tree,
    /// test if it’s a valid solution as we go along.
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