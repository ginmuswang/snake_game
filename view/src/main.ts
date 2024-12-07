import init, { World, Direction, GameStatus } from 'snake_game';
import { random } from '../utils/random';

init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 32;
  const WORLD_SIZE = WORLD_WIDTH * WORLD_WIDTH;
  const SNAKE_SPAWN_IDX = random(WORLD_SIZE);
  const FPS = 10;
  let currentDirection: Direction = Direction.Right;
  let snakeSize: number = 5;
  const world = World.new(
    WORLD_WIDTH,
    SNAKE_SPAWN_IDX,
    currentDirection,
    snakeSize
  );
  const worldWidth = world.width();
  const canvas = document.getElementById('snake-canvas') as HTMLCanvasElement;
  const ctx = canvas.getContext('2d')!;
  canvas.height = CELL_SIZE * worldWidth;
  canvas.width = CELL_SIZE * worldWidth;

  /**添加键盘点击事件 */
  document.addEventListener('keydown', (event) => {
    const keyMapDirection: Record<string, Direction | undefined> = {
      ArrowLeft: Direction.Left,
      KeyA: Direction.Left,
      ArrowUp: Direction.Up,
      KeyW: Direction.Up,
      ArrowRight: Direction.Right,
      KeyD: Direction.Right,
      ArrowDown: Direction.Down,
      KeyS: Direction.Down,
    };
    const targetDirection = keyMapDirection[event.code];
    if (typeof targetDirection !== 'undefined') {
      if (targetDirection !== currentDirection) {
        world.change_snake_dir(targetDirection);
        currentDirection = targetDirection;
      }
    }
  });

  function drawWorld() {
    ctx.beginPath();
    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }
    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_len()
    );

    snakeCells
      .filter((cellIdx, i) => !(i > 0 && cellIdx === snakeCells[0]))
      .forEach((cellIdx, i) => {
        const col = cellIdx % worldWidth;
        const row = Math.floor(cellIdx / worldWidth);

        ctx.fillStyle = i == 0 ? '#7878bd' : '#000';
        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
        ctx.stroke();
      });
  }

  function drawReward() {
    const idx = world.reward_cell();
    if (idx) {
      const col = idx % worldWidth;
      const row = Math.floor(idx / worldWidth);
      ctx.beginPath();
      ctx.fillStyle = '#FF0000';
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      ctx.stroke();
    }
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
  }

  function update(timer?: number) {
    if (timer) {
      clearTimeout(timer);
    }
    return () => {
      let t = setTimeout(() => {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        world.step();
        paint();
        requestAnimationFrame(update(t));
      }, 1000 / FPS);
    };
  }

  paint();
  update()();
});
