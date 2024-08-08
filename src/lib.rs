#![no_std]

use gstd::{debug, exec, msg, prelude::*};
use pebbles_game_io::*;

/// 表示游戏的状态
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebbleGame {
    pub pebbles_count: u32,          // 总的弹珠数
    pub max_pebbles_per_turn: u32,   // 每轮最大弹珠数
    pub pebbles_remaining: u32,      // 剩余弹珠数
    pub program_lastmove: u32,       // 上一轮程序的移动数
    pub difficulty: DifficultyLevel, // 难度等级
    pub first_player: Player,        // 首先移动的玩家
    pub winner: Option<Player>,      // 胜利者
}

/// 生成随机数
pub fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("生成随机数失败");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

/// 生成程序的移动数
pub fn program_turn_gen(difficulty: DifficultyLevel, max_per_turn: u32) -> u32 {
    match difficulty {
        DifficultyLevel::Easy => (get_random_u32() % max_per_turn) + 1,
        DifficultyLevel::Hard => {
            let mut count = get_random_u32() % max_per_turn;
            if count / 2 < max_per_turn {
                count = get_random_u32() % max_per_turn;
            }
            count + 1
        }
    }
}

impl PebbleGame {
    /// 用户的移动操作
    fn user_move(&mut self, count: u32) {
        // 检查用户移动的弹珠数是否在有效范围内
        if !self.is_valid_move(count) {
            panic!(
                "无效的移动。你可以移除 {} 个弹珠。",
                self.max_pebbles_per_turn
            );
        }

        // 检查剩余弹珠数是否足够
        if count > self.pebbles_remaining {
            panic!("无效的移动。剩余弹珠不足。");
        }

        self.pebbles_remaining -= count;

        // 检查游戏是否结束
        if self.pebbles_remaining == 0 {
            self.winner = Some(Player::User);
            msg::reply(PebblesEvent::Won(Player::User), 0).unwrap();
        } else {
            self.program_move();
        }
    }

    /// 验证移动是否合法
    fn is_valid_move(&self, count: u32) -> bool {
        (1..=self.max_pebbles_per_turn).contains(&count)
    }

    /// 程序的移动操作
    fn program_move(&mut self) {
        let count = self.calculate_program_move();
        let count = count.min(self.pebbles_remaining);
        self.pebbles_remaining -= count;
        self.program_lastmove = count;

        // 检查游戏是否结束
        if self.pebbles_remaining == 0 {
            self.winner = Some(Player::Program);
            msg::reply(PebblesEvent::Won(Player::Program), 0).unwrap();
        } else {
            debug!(
                "当前剩余弹珠数: {}",
                self.pebbles_count - self.pebbles_remaining
            );
            msg::reply(PebblesEvent::CounterTurn(count), 0).unwrap();
        }
    }

    /// 计算程序的移动数
    fn calculate_program_move(&self) -> u32 {
        if self.max_pebbles_per_turn != 1 {
            program_turn_gen(self.difficulty.clone(), self.max_pebbles_per_turn)
        } else {
            1
        }
    }

    /// 重置游戏状态
    fn restart(
        &mut self,
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    ) {
        self.difficulty = difficulty;
        self.pebbles_count = pebbles_count;
        self.max_pebbles_per_turn = max_pebbles_per_turn;
        self.pebbles_remaining = self.pebbles_count;
        self.winner = None;
        self.program_lastmove = 0;
        self.first_play();
    }

    /// 决定谁先移动
    fn first_play(&mut self) {
        if get_random_u32() % 2 == 0 {
            self.first_player = Player::User;
            msg::reply(PebblesEvent::CounterTurn(0), 0).unwrap();
        } else {
            self.first_player = Player::Program;
            self.program_move();
        }
    }
}

static mut PEBBLE_GAME: Option<PebbleGame> = None;

/// 初始化游戏
#[no_mangle]
extern "C" fn init() {
    let config: PebblesInit = msg::load().expect("无法解码初始化配置");
    assert!(
        config.max_pebbles_per_turn <= config.pebbles_count,
        "初始化参数无效。"
    );

    let mut game = PebbleGame {
        pebbles_count: config.pebbles_count,
        max_pebbles_per_turn: config.max_pebbles_per_turn,
        difficulty: config.difficulty,
        pebbles_remaining: config.pebbles_count,
        ..Default::default()
    };
    game.first_play();
    debug!(
        "游戏初始化: 总弹珠数={}, 每轮最大弹珠数={}, 剩余弹珠数={}",
        game.pebbles_count, game.max_pebbles_per_turn, game.pebbles_remaining
    );
    unsafe { PEBBLE_GAME = Some(game) };
}

/// 处理游戏操作
#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("无法加载操作");
    let game = unsafe { PEBBLE_GAME.get_or_insert(Default::default()) };
    game.program_lastmove = 0;

    match action {
        PebblesAction::Turn(count) => {
            game.user_move(count);
        }
        PebblesAction::GiveUp => {
            game.program_move();
        }
        PebblesAction::Restart {
            difficulty,
            pebbles_count,
            max_pebbles_per_turn,
        } => {
            game.restart(difficulty, pebbles_count, max_pebbles_per_turn);
        }
    }
}

/// 返回游戏状态
#[no_mangle]
extern "C" fn state() {
    let gmst = unsafe { PEBBLE_GAME.get_or_insert(Default::default()) };
    msg::reply(gmst.clone(), 0).expect("无法返回游戏状态");
}
