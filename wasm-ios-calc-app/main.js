import './style.css'
import symbols from './symbols';
import fitty from 'fitty'
import { Calculator } from 'wasm-ios-calc';

/**
 * Create an array of numbers, starting from 1.
 * @param {number} size 
 * @returns {number[]}  
 */
function range(N) {
  var a = Array(N), b = 0;
  while (b < N) a[b++] = b;
  return a;
}

/**
 * Helper function to set css grid row and column.
 * @param {HTMLElement} el 
 * @param {number} row 
 * @param {number} column 
 */
function setGridPos(el, row, column) {
  el.style.gridRow = row;
  el.style.gridColumn = column;
}

const buttonConfig = {
  'c': {
    innerHTML: 'AC',
  },
  '±': {
    innerHTML: symbols['±'],
  },
  '%': {
    innerHTML: symbols['%'],
  },
  '/': {
    innerHTML: symbols['/'],
  },
  '*': {
    innerHTML: symbols['*'],
  },
  '-': {
    innerHTML: symbols['-'],
  },
  '+': {
    innerHTML: symbols['+'],
  },
  '=': {
    innerHTML: symbols['='],
  }
};

/**
 * Create the button elements in the calculator's keypad.
 * @param {HTMLDivElement} kepadEl 
 * @param {(id: string | number) => void} buttonClickFn 
 */
function createKeypadButtons(kepadEl, buttonClickFn) {

  const buttonClassName = 'button';

  /**
   * Creates a generic keypad button element. 
   * @param {string | number} to show in button and assign as id. 
   * @returns {HTMLButtonElement} Button element.
   */
  const createButtonEl = (id) => {
    const buttonEl = document.createElement(buttonClassName);
    buttonEl.classList.add('button');

    const contentEl = document.createElement('div');
    contentEl.classList.add('inner');
    buttonEl.appendChild(contentEl);

    if (buttonConfig[id]) {
      contentEl.innerHTML = buttonConfig[id].innerHTML;
    } else {
      contentEl.innerHTML = id;
    }

    buttonEl.id = id;
    buttonEl.onclick = () => buttonClickFn(id);
    return buttonEl;
  }

  // Function Buttons
  const fnButtons = ['c', '±', '%'];

  for (const [index, fn] of fnButtons.entries()) {
    const buttonEl = createButtonEl(fn);
    buttonEl.classList.add('function');
    setGridPos(buttonEl, 1, index + 1);
    kepadEl.appendChild(buttonEl);
  }

  // Operator Buttons
  const operatorButtons = ['/', '*', '-', '+', '='];

  for (const [index, operator] of operatorButtons.entries()) {
    const buttonEl = createButtonEl(operator);
    buttonEl.classList.add('operator');
    setGridPos(buttonEl, index + 1, 4);
    kepadEl.appendChild(buttonEl);
  }

  // Decimal button
  const decimalButtonEl = createButtonEl('.');
  decimalButtonEl.classList.add('digit');
  setGridPos(decimalButtonEl, 5, 3);
  kepadEl.appendChild(decimalButtonEl);

  // Number Buttons
  for (let number of range(10)) {
    // Range should go from 0 to 9.
    number -= 1;

    const buttonEl = createButtonEl(number);
    buttonEl.classList.add('digit');

    // Number zero spans two columns
    if (number == 0) {
      buttonEl.style.gridRow = 5;
      buttonEl.style.gridColumn = '1 / span 2';
    } else {
      setGridPos(buttonEl, Math.ceil((10 - number) / 3) + 1, (9 % 3));
    }
    kepadEl.appendChild(buttonEl);
  }
}

/**
 * Outputs to the calculator's display.
 * @param {Calculator} calculator 
 */
function updateDisplay(calculator) {
  const displayEl = document.getElementById('display')

  // Don't update if output is unchanged.
  if (!displayEl || displayEl.innerText == calculator.output) {
    return;
  }

  displayEl.innerText = calculator.output;

  // Fit the output font size to the display.
  fitty(displayEl, {
    multiLine: false,
    maxSize: 110,
  });
}

/* Reference to currently active operator button element. */
let activeOperatorBtnEl = undefined;


function animateButtonPressDown(el) {

}

/**
 * Update calculator view.
 * @param {Calculator} calculator 
 */
function update(calculator) {
  updateDisplay(calculator);

  /* Update clear button. */
  const clearBtnEl = document.getElementById('c');
  if (clearBtnEl) {
    clearBtnEl.firstElementChild.innerHTML = calculator.showAllClear ? 'AC' : 'C';
  }

  /* Reset active operator element. */
  activeOperatorBtnEl?.classList.remove('active');

  if (calculator.activeOperator) {
    activeOperatorBtnEl = document.getElementById(calculator.activeOperator);

    /* Style active operator. */
    activeOperatorBtnEl?.classList.add('active');
  }
}

function handleButtonPress(calculator, key) {
  calculator.buttonPressed(key.toString());
  update(calculator);
}

/**
 * 
 * @param {Calculator} calculator 
 * @param {string} key 
 */
function handleKeyPress(calculator, key) {
  let keyString = key.toString();

  const validKeys = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '/', '*', '-', '+', 'c', '±', '%', '='];

  if (!validKeys.includes(keyString)) {
    return;
  }

  const pressedButtonEl = document.getElementById(keyString);
  if (pressedButtonEl) {
    pressedButtonEl.classList.add('pressed');
    pressedButtonEl.ontransitionend = () => {
      pressedButtonEl.classList.remove('pressed');
    };
  }

  handleButtonPress(calculator, keyString);
}

function create() {
  /* Create wasm Calculator instance. */
  const calculator = new Calculator();

  /* Create keypad buttons, supply key press callback. */
  createKeypadButtons(document.getElementById("keypad"), (id) => {
    handleButtonPress(calculator, id)
  });
  1
  window.addEventListener("keyup", (event) => {
    event.preventDefault();
    handleKeyPress(calculator, event.key)
  });

  /* Initialise calculator */
  update(calculator);
}

/* Run */
create();

