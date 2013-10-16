#[desc = "A clone of the aw interpreter in rust"];
#[license = "MIT"];

use std::hashmap::HashMap;

type Env = HashMap<~str, int>;

enum Expr {
  Number(int),
  Var(~str),
  Plus(~Expr, ~Expr),
  Minus(~Expr, ~Expr),
  Times(~Expr, ~Expr)
}

fn lookup(env: &Env, x: ~str) -> int {
  // TODO: figure out how to error out ourselves here
  *env.find(~x).get()
}

fn interp(expr: ~Expr, lookupEnv: &Env) -> int {
  match *expr {
    Plus(a,b) => interp(a, lookupEnv) + interp(b, lookupEnv),
    Minus(a,b) => interp(a, lookupEnv) - interp(b, lookupEnv),
    Times(a,b) => interp(a, lookupEnv) * interp(b, lookupEnv),
    Number(x) => x,
    Var(x) => lookup(lookupEnv, x)
  }
}

#[test]
fn number() {
  let lookupEnv: ~Env = ~HashMap::new();
  let expr: ~Expr = ~Number(5);
  assert!(interp(expr, lookupEnv) == 5);
}

#[test]
fn var() {
  let mut lookupEnv: ~Env = ~HashMap::new();
  lookupEnv.insert(~"x", 0);
  let expr: ~Expr = ~Var(~"x");
  assert!(interp(expr, lookupEnv) == 0);
}

// 5 + 5
#[test]
fn plus() {
  let mut lookupEnv: ~Env = ~HashMap::new();
  lookupEnv.insert(~"x", 5);
  let expr: ~Expr = ~Plus(~Var(~"x"), ~Number(5));
  assert!(interp(expr, lookupEnv) == 10);
}

// 5 + 5 - 3
#[test]
fn minus() {
  let mut lookupEnv: ~Env = ~HashMap::new();
  lookupEnv.insert(~"x", 5);
  let expr: ~Expr = ~Minus(~Plus(~Var(~"x"), ~Number(5)), ~Number(3));
  assert!(interp(expr, lookupEnv) == 7);
}

// 3 * (5 + 5 - 3)
#[test]
fn times() {
  let mut lookupEnv: ~Env = ~HashMap::new();
  lookupEnv.insert(~"x", 5);
  let expr: ~Expr = ~Times(~Minus(~Plus(~Var(~"x"), ~Number(5)), ~Number(3)), ~Number(3));
  assert!(interp(expr, lookupEnv) == 21);
}
