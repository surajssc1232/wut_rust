javascript
javascript
javascript
javascript
function add(x, y) {
	return x + y;
}

function subtract(x, y) {
  return x - y;
}

function multiply(x, y) {
  return x * y;
}

function getFileSize(file) {
  return file.size;
}

class Calculator {
  constructor() {
  }

  add(x, y) {
    return x + y;
  }

  subtract(x, y) {
    return x - y;
  }

  multiply(x, y) {
    return x * y;
  }

  divide(x, y) {
    if (y === 0) {
      return "Cannot divide by zero!";
    }
    return x / y;
  }
}