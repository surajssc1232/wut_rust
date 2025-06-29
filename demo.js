javascript
javascript
// Function to add two numbers
function add(x, y) {
	return x + y; // Returns the sum of x and y
}

// Function to subtract two numbers
function subtract(x, y) {
  return x - y; // Returns the difference between x and y
}

// Function to multiply two numbers
function multiply(x, y) {
  return x * y; // Returns the product of x and y
}

// Calculator class
class Calculator {
  constructor() {
    // Initialize calculator properties if needed
  }

  // Method to add two numbers
  add(x, y) {
    return x + y; // Returns the sum of x and y
  }

  // Method to subtract two numbers
  subtract(x, y) {
    return x - y; // Returns the difference between x and y
  }

  // Method to multiply two numbers
  multiply(x, y) {
    return x * y; // Returns the product of x and y
  }

  // Method to divide two numbers
  divide(x, y) {
    if (y === 0) {
      return "Cannot divide by zero!"; // Returns an error message if y is zero
    }
    return x / y; // Returns the quotient of x and y
  }
}