import './style.css'

const frickOne = "frick-[--[----->+<]>----.-.++.++++++++.----.+++++++++++.---.++++++.-------.----------.[----->+++<]>--.[-->+<]>+++.++[->+++<]>+.+++++++++++.-------------..+++++++++.[-->+<]>++++.]";
const frickTwo = "frick-[--[----->+<]>-----.+++++++++++++.-----.++++.------------.--[--->+<]>-.-----------.++++++.-.[-->+<]>+++.[-->+<]>+++.--[->++++<]>--.[->+++<]>-.--[--->+<]>---.[--->+<]>++.+++++..+++[->++<]>+.+++++++++++++.-----.++++.------------.--[--->+<]>-.+++[->+++<]>.[->+++<]>--.--[----->+<]>+.+++++.---------.+++++++.++++[->+++<]>.--[->+++<]>.-[-->+++<]>-.]";
const frickError = "frick-[!]";

document.querySelector<HTMLDivElement>('#app')!.innerHTML = /* html */`
  <h1 class="text-3xl font-bold underline ${frickOne} ${frickTwo} ${frickError}">Hello world!</h1>
`;
