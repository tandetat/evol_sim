import * as sim from "lib-simulation-wasm";

const simulation = new sim.Simulation();

const viewport = document.getElementById('viewport');
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;


const viewportScale = window.devicePixelRatio || 1;

viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;

viewport.style.width = viewportWidth + 'px';
viewport.style.height = viewportHeight + 'px';

const context = viewport.getContext('2d');

context.scale(viewportScale, viewportScale);
context.fillStyle = 'rgb(0, 0, 0)';

CanvasRenderingContext2D.prototype.drawTriangle = 
    function (x, y, size, rotation ) {
        const interiorAng = 2.0/3.0 * Math.PI;
        const doubleAng = 2.0 * interiorAng;

        const coeff = 1.5;
        this.beginPath();

        this.moveTo(
            x + Math.cos(rotation) * size * coeff,
            y + Math.sin(rotation) * size * coeff,
        );
        this.lineTo(
            x + Math.cos(rotation + interiorAng) * size,
            y + Math.sin(rotation + interiorAng) * size,
        );
        
        this.lineTo(
            x + Math.cos(rotation + doubleAng) * size,
            y + Math.sin(rotation + doubleAng) * size,
        );
        this.lineTo(
            x + Math.cos(rotation) * size * coeff,
            y + Math.sin(rotation) * size * coeff,
        );
        

        this.fillStyle = 'rgb(255, 255, 255)'; // A nice white color
        this.fill();
        this.stroke();
};

CanvasRenderingContext2D.prototype.drawCircle = 
    function(x,y, radius) {
        this.beginPath();

        this.arc(x,y, radius, 0, 2.0 * Math.PI);

        this.fillStyle = 'rgb(0, 150, 0)';
        this.fill();
};
function redraw() {
        context.clearRect(0, 0, viewportWidth, viewportHeight);


        const world = simulation.world();
        simulation.step();
        
        for (const animal of simulation.world().animals) {
            context.drawTriangle(
                animal.x * viewportWidth,
                animal.y * viewportHeight,
                0.01 * viewportWidth,
                animal.rotation,
            );
        }

        
        requestAnimationFrame(redraw);

        for (const food of world.foods) { 
            context.drawCircle(
                food.x * viewportWidth,
                food.y * viewportHeight,
                (0.01 / 2.0) * viewportWidth,
            );
        }
}
    
redraw();