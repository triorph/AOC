cards
  = cards:card|..,"\n"| "\n"* { return cards; }
card
  = "Card" _ id:number ":" _ have:number|.., _| _ "|" _ winning:number|..,_| { return new Card(id, have, winning); }
number 
  = digits:[0-9]+ { return parseInt(digits.join(""), 10); }
_ 
  = [ \t]+
