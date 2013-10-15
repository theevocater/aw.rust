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

fn main() {
  // TODO: figure out how to literal this
  let mut lookupEnv: ~Env = ~HashMap::new();
  lookupEnv.insert(~"x", 5);

  let mut expr: ~Expr;

  expr = ~Number(5);
  println(interp(expr, lookupEnv).to_str());

  expr = ~Var(~"x");
  println(interp(expr, lookupEnv).to_str());

  // 5 + 5
  expr = ~Plus(~Var(~"x"), ~Number(5));
  println(interp(expr, lookupEnv).to_str());

  // 5 + 5 - 3
  expr = ~Minus(~Plus(~Var(~"x"), ~Number(5)), ~Number(3));
  println(interp(expr, lookupEnv).to_str());

  // 3 * (5 + 5 - 3)
  expr = ~Times(~Minus(~Plus(~Var(~"x"), ~Number(5)), ~Number(3)), ~Number(3));
  println(interp(expr, lookupEnv).to_str());
}
