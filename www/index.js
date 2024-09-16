import { Area, Bird, bird_radius } from "birdsim";
import { memory } from "birdsim/birdsim_bg";

const RED = 'red';
const ORANGE = 'rgba(250, 187, 0, 0.44)';
const GREEN = 'rgba(21, 203, 0, 0.49)'
const BLACK = 'black';

const area = new Area();
const width = area.width();
const height = area.height();
const radius = bird_radius();

const canvas = document.getElementById("canva");
canvas.height = height;
canvas.width = width;

const addBirdButton = document.getElementById("add-bird");
addBirdButton.addEventListener("click", event => {
    addBird();
    draw();
});

const ctx = canvas.getContext('2d');

const addBird = () => {
    area.add_bird();
}

const draw = () => {
    console.log("number of birds : " + area.nb_birds())

    ctx.reset();

    area.get_birds().forEach((bird, index) => {
        drawBird(ctx, bird)
        console.log(`bird ${index}`)
    })
}

const drawBird = (ctx, bird) => {
    ctx.fillStyle = ORANGE;
    ctx.ellipse(bird.coord_x(), bird.coord_y(), radius, radius, 0, 0, 2 * Math.PI);
    ctx.fill();

    ctx.lineWidth = 2;
    ctx.strokeStyle = RED;
    ctx.beginPath();
    ctx.moveTo(bird.coord_x(), bird.coord_y());
    ctx.lineTo(bird.direction_line_stop().x(), bird.direction_line_stop().y());
    ctx.stroke();

    ctx.fillStyle = BLACK;
    ctx.ellipse(bird.coord_x(), bird.coord_y(), 5, 5, 0, 0, 2 * Math.PI);
    ctx.fill();

    ctx.closePath();
}

draw();