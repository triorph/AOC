games
  = games:g|.., _ '\n' _| '\n'* { return games; }
g
  = "Game " id:number ":" _ rounds:r|.., _ ';' _| { return new Game(id, rounds) }
r
  = entries:entry|.., _ "," _| { return new Round(entries); }
entry
  = quantity:number _ colour:word { return [quantity, colour]; }
word 
  = $[a-z]i+
number 
  = digits:[0-9]+ { return parseInt(digits.join(""), 10); }
_ 
  = [ \t]*
