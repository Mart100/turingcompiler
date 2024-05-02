export function getCodeExamples() {
  return {
    "Math Operation": `
let a = (22 - 8) * 4 + 5;
let b = a == 61;
return b;`,

    "If Statement": `
let a = (22 - 8) * 4 + 6;
let b = a == 61;
let c = 0;
if b {
    c = 12;
} else {
    c = 2;
};
return c;`,

    "While Loop": `
let a = 4;
let b = 2;
while (a > 0) {
    a = a - 1;
    b = b * 2;
};
return b;`,

    "Function ": `
fn add(a, b) {
    return a + b;
};
fn main() {
    let c = add(1,8);
    let d = add(2,3);
    let e = add(c,d);
    return e;
};`,
  };
}
