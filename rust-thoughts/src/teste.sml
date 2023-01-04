fun eval Const(X) = true
  | eval Var(X) = false
  | eval _ = false
