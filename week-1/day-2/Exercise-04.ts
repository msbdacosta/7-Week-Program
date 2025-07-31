// Exercise 4: Real-world Example — Tip Calculator

function calculatorTip(total: number, percent: number): number {
  return total * (percent / 100);
}

const tip = calculatorTip(100, 15);
console.log(`The tip is $${tip}.`);
