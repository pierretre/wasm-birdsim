import { Area, Bird, bird_radius, DirectionVector } from "birdsim";

const RED = 'red';
const ORANGE = 'rgba(250, 187, 0, 0.44)';
const GREEN = 'rgba(21, 203, 0, 0.49)'
const BLACK = 'black';

const area = new Area();
const width = area.width();
const height = area.height();
const radius = area.bird_radius();

const canvas = document.getElementById("canva");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext('2d');

const addBirdButton = document.getElementById("add-bird");

addBirdButton.addEventListener("click", event => {
    addBird();
});


function addBird() {
    area.add_bird();
}

function renderLoop() {

    area.tick();

    let nb = area.nb_birds();
    console.log("number of birds : " + nb);

    ctx.reset();

    area.get_birds().forEach((bird, index) => {
        drawBird(bird)
    });
}

function drawBird(bird) {

    const birdx = bird.coord_x();
    const birdy = bird.coord_y();
    const end = bird.direction_line_stop();

    ctx.beginPath()
    ctx.fillStyle = ORANGE;
    ctx.ellipse(birdx, birdy, radius, radius, 0, 0, 2 * Math.PI);
    ctx.fill();
    ctx.closePath();

    ctx.beginPath()
    ctx.lineWidth = 2;
    ctx.strokeStyle = RED;
    ctx.beginPath();
    ctx.moveTo(birdx, birdy);
    ctx.lineTo(end.x(), end.y());
    ctx.stroke();
    ctx.closePath();

    ctx.beginPath()
    ctx.fillStyle = BLACK;
    ctx.ellipse(birdx, birdy, 5, 5, 0, 0, 2 * Math.PI);
    ctx.fill();
    ctx.closePath();
}

setInterval(() => {
    requestAnimationFrame(renderLoop);
}, 1000 / 10);