import { NewtonFractal, Complex, Poly } from "wasm-newton-fractal";
import { memory } from "wasm-newton-fractal/wasm_newton_fractal_bg";

const HEIGHT_IMG = 1024;
const WIDTH_IMG = 1024;
const X1 = -2.0;
const X2 = +2.0;
const Y1 = -2.0;
const Y2 = +2.0;
const ITERATIONS = 255;
const ITER_COEF = 17;
const COLORS = [
    [255, 0, 0],
    [0, 255, 0],
    [0, 0, 255],
    [0, 255, 255],
    [255, 255, 0],
    [255, 0, 255],
    [255, 255, 255],
    [0, 128, 255],
    [255, 128, 0],
    [128, 0, 255],
];

const rootsInput = document.getElementById("roots");
rootsInput.value = "(1,0); (-0.5,0.86602); (-0.5,-0.86602)"; // x^3 - 1
rootsInput.addEventListener("input", updatePolynomial);

function parseRoots() {
    const input = rootsInput.value;
    const regex = /\(-?\d+(.\d+)?,-?\d+(.\d+)?\)/g;
    const matches = input.match(regex);

    if (!matches) return [];
    const roots = matches.map((c) => {
        let parts = c.split(",");
        let real = parseFloat(parts[0].substring(1));
        let imag = parseFloat(parts[1].substring(0, parts[1].length - 1));
        return Complex.new(real, imag);
    });
    return roots;
}

const polynomialDiv = document.getElementById("polynomial-box");

function updatePolynomial() {
    const polyString = Poly.from_roots(parseRoots()).to_str();
    polynomialDiv.innerHTML = "Polynomial: " + polyString;
}
updatePolynomial();

const canvas = document.getElementById("fractal-canvas");
canvas.height = HEIGHT_IMG;
canvas.width = WIDTH_IMG;
const ctx = canvas.getContext("2d");

const drawFractal = () => {
    let fractal = NewtonFractal.new(
        parseRoots(),
        X1,
        X2,
        Y1,
        Y2,
        HEIGHT_IMG,
        WIDTH_IMG,
        ITERATIONS
    );

    const colorsPtr = fractal.colors();
    const colorInfo = new Uint8Array(
        memory.buffer,
        colorsPtr,
        2 * WIDTH_IMG * HEIGHT_IMG
    );

    for (let y = 0; y < HEIGHT_IMG; y++) {
        for (let x = 0; x < WIDTH_IMG; x++) {
            const idx = 2 * (y * WIDTH_IMG + x);
            const clr = colorInfo[idx + 0];
            const itr = colorInfo[idx + 1];
            const rgb = COLORS[clr % COLORS.length];
            const frc = 1.0 - (itr * ITER_COEF) / ITERATIONS;
            ctx.fillStyle = `rgb(${frc * rgb[0]}, ${frc * rgb[1]}, ${
                frc * rgb[2]
            })`;
            ctx.fillRect(x, y, 1, 1);
        }
    }
};
drawFractal();

const renderButton = document.getElementById("render-btn");
renderButton.addEventListener("click", drawFractal);
