import init, { format, run_program } from "./out/ram_machine.js";

await init();

const code = document.querySelector("#code");
const errorMsg = document.querySelector("#error-msg");
const out = document.querySelector("#out");
const registers = document.querySelector("#registers");
const formatBtn = document.querySelector("#format-btn");
const runBtn = document.querySelector("#run-btn");
const addRegBtn = document.querySelector("#add-reg-btn");

const onRun = () => {
  try {
    errorMsg.innerText = "";
    const regs = {};
    for (const row of registers.children) {
      regs[row.querySelector(".reg").value] =
        row.querySelector(".reg-val").value;
    }
    const res = run_program(code.value, regs);

    out.innerText = res;
  } catch (err) {
    out.innerText = "-";
    errorMsg.innerText = err;
  }
};

const onFormat = () => {
  try {
    errorMsg.innerText = "";
    code.value = format(code.value);
  } catch (err) {
    errorMsg.innerText = err;
  }
};

const createRegister = (name = "", value = "") => {
  const tr = document.createElement("tr");
  const regTd = document.createElement("td");
  const valTd = document.createElement("td");
  const regInput = document.createElement("input");
  const valInput = document.createElement("input");

  regInput.value = name;
  valInput.value = value;
  regInput.classList.add("reg");
  valInput.classList.add("reg-val");

  regTd.appendChild(regInput);
  valTd.appendChild(valInput);

  tr.append(regTd, valTd);

  registers.appendChild(tr);
};

const revRam = `
N0: clr RY 
    clr RZ

N1: RX jmp0 N2
    RX jmp1 N3
    jmp NEND

N2: add0 RY
    del RX
    jmp N4

N3: add1 RY
    del RX
    jmp N4

N4: RZ jmp0 N5
    RZ jmp1 N6
    RZ <- RY
    clr RY
    jmp N1

N5: del RZ
    add0 RY
    jmp N4

N6: del RZ
    add1 RY
    jmp N4

NEND: RX <- RZ
      clr RY
      clr RZ
      continue`;

code.value = revRam;
createRegister("RX", "010111");

onFormat();
onRun();

runBtn.addEventListener("click", onRun);
formatBtn.addEventListener("click", onFormat);
addRegBtn.addEventListener("click", () => createRegister());
