enum Expr {
  Number(int),
  Plus(~Expr, ~Expr),
  Minus(~Expr, ~Expr),
  Times(~Expr, ~Expr)
}

fn interp(expr: ~Expr) -> int {
  match *expr {
    Plus(a,b) => interp(a) + interp(b),
    Minus(a,b) => interp(a) - interp(b),
    Times(a,b) => interp(a) * interp(b),
    Number(x) => x
  }
}

fn main() {
  let mut expr: ~Expr;
  expr = ~Number(5);
  println(interp(expr).to_str());

  // 5 + 5
  expr = ~Plus(~Number(5), ~Number(5));
  println(interp(expr).to_str());

  // 5 + 5 - 3
  expr = ~Minus(~Plus(~Number(5), ~Number(5)), ~Number(3));
  println(interp(expr).to_str());

  // 3 * (5 + 5 - 3)
  expr = ~Times(~Minus(~Plus(~Number(5), ~Number(5)), ~Number(3)), ~Number(3));
  println(interp(expr).to_str());
}
