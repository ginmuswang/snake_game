use js_sys::Math;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
#[wasm_bindgen]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[wasm_bindgen]
#[derive(Clone, PartialEq)]
pub struct SnakeCell(usize);
#[wasm_bindgen]
pub struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}
#[wasm_bindgen]
impl Snake {
    pub fn new(spawn_index: usize, direction: Direction, size: usize) -> Self {
        let mut body: Vec<_> = vec![];
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }
        Snake { body, direction }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize, direction: Direction, snake_size: usize) -> Self {
        let size = width * width;
        let snake = Snake::new(snake_idx, direction, snake_size);
        let reward_cell = Self::gen_reward_cell(size, &snake.body);
        World {
            width,
            size,
            snake,
            reward_cell,
            status: Some(GameStatus::Played),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        assert!(snake_body.len() < max, "已经没有多的空间可以生成奖励块了");
        loop {
            let temp_reward_cell = Math::floor(Math::random() * (max as f64)) as usize;
            if !snake_body.contains(&SnakeCell(temp_reward_cell)) {
                break Some(temp_reward_cell);
            }
        }
    }

    /// `snake_head_idx`返回蛇头所处的index
    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    /** `step`根据蛇的的前进方向，找出蛇的下一个位置，并将下一个位置更新到自身`self.snake.body`中
     *  #Example
     *  ```
     *  //直接调用即可
     *  self.step()
     *  ```
     * */
    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => self.move_whole_snake(),
            Some(GameStatus::Lost) => {}
            Some(GameStatus::Won) => {}
            None => {}
        }
    }

    /** `move_whole_snake`移动整个蛇的位置，具体做法就是并找到每一个SnakeCell的下一个位置，然后更新到自身`self.snake.body`中
     *  #Example
     *  ```
     *  //直接调用即可
     *  self.move_whole_snake()
     *  ```
     * */
    fn move_whole_snake(&mut self) {
        let temp = self.snake.body.clone();
        let next_cell = self.gen_next_snake_cell(&self.snake.direction);
        self.snake.body[0] = next_cell;

        let len = self.snake.body.len();

        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0)
        }
        if self.snake.body[1..].contains(&self.snake.body[0]) {
            self.status = Some(GameStatus::Lost);
        };

        if self.reward_cell == Some(self.snake_head_idx()) {
            if self.snake.body.len() < self.size {
                self.snake.body.push(SnakeCell(self.snake.body[1].0));
                self.reward_cell = Self::gen_reward_cell(self.size, &self.snake.body);
            } else {
                self.reward_cell = None;
                self.status = Some(GameStatus::Won);
            }
        }
    }

    ///生成下一个蛇单元
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        match direction {
            Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
            Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
            Direction::Up => SnakeCell((snake_idx - self.width) % self.size),
            Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
        }
    }

    /// 获得蛇数组的首地址指针
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    /// 获得蛇数组的长度
    pub fn snake_len(&self) -> usize {
        self.snake.body.len()
    }

    /// 改变蛇的方向
    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if next_cell.0 != self.snake.body[1].0 {
            self.snake.direction = direction;
        }
    }
}
