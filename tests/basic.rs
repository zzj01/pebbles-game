use gstd::Encode;
use gtest::{Program, System};
use pebbles_game::*;
use pebbles_game_io::*;

const USERS: &[u64] = &[3, 4, 5]; // 定义用户ID

/// 初始化游戏状态
fn init_game(sys: &System, total: u32, turn_max: u32) {
    sys.init_logger(); // 初始化日志

    let game = Program::current_opt(sys); // 获取当前程序实例
    let res = game.send(
        USERS[0],
        PebblesInit {
            pebbles_count: total,              // 设置弹珠总数
            max_pebbles_per_turn: turn_max,    // 设置每轮最大弹珠数
            difficulty: DifficultyLevel::Easy, // 设置难度等级
        },
    );

    assert!(!res.main_failed()); // 确保初始化操作没有失败

    // 读取游戏状态
    let gm: PebbleGame = game.read_state(0).expect("无效的状态。");
    assert_eq!(gm.pebbles_count, total); // 检查总弹珠数
    assert_eq!(gm.max_pebbles_per_turn, turn_max); // 检查每轮最大弹珠数
    match gm.first_player {
        Player::User => assert_eq!(gm.pebbles_count, gm.pebbles_remaining), // 如果用户先手，检查剩余弹珠数
        Player::Program => assert_eq!(gm.pebbles_count, gm.pebbles_remaining + gm.program_lastmove), // 如果程序先手，检查剩余弹珠数加上程序上一次移动的弹珠数
    }
}

/// 测试初始化成功
#[test]
fn init_successed() {
    let sys = System::new();
    sys.init_logger(); // 初始化日志

    let game = Program::current_opt(&sys); // 获取当前程序实例
    let res = game.send(
        USERS[0],
        PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 9,
            difficulty: DifficultyLevel::Easy,
        },
    );
    assert!(!res.main_failed()); // 确保初始化操作没有失败
}

/// 测试初始化失败
#[test]
fn init_failed() {
    let sys = System::new();
    sys.init_logger(); // 初始化日志

    let game = Program::current_opt(&sys); // 获取当前程序实例
    let res = game.send(
        USERS[0],
        PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 11, // 设置无效的每轮最大弹珠数
            difficulty: DifficultyLevel::Easy,
        },
    );
    assert!(res.main_failed()); // 确保初始化操作失败
}

/// 测试用户移动操作
#[test]
fn user_move() {
    let sys = System::new();
    init_game(&sys, 101, 3); // 初始化游戏
    let game = sys.get_program(1).unwrap(); // 获取游戏实例
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。"); // 读取游戏状态
    let mut remaing = gmstate.pebbles_remaining; // 获取剩余弹珠数

    // 用户进行移动操作
    let res = game.send(USERS[0], PebblesAction::Turn(1));
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。");
    assert!(res.contains(&(
        USERS[0],
        PebblesEvent::CounterTurn(gmstate.program_lastmove).encode()
    )));
    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - 1 - gmstate.program_lastmove
    );

    // 用户再次进行移动操作
    remaing = gmstate.pebbles_remaining;
    let res = game.send(USERS[0], PebblesAction::Turn(2));
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。");
    assert!(res.contains(&(
        USERS[0],
        PebblesEvent::CounterTurn(gmstate.program_lastmove).encode()
    )));
    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - 2 - gmstate.program_lastmove
    );

    // 用户再次进行移动操作
    remaing = gmstate.pebbles_remaining;
    let res = game.send(USERS[0], PebblesAction::Turn(3));
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。");
    assert!(res.contains(&(
        USERS[0],
        PebblesEvent::CounterTurn(gmstate.program_lastmove).encode()
    )));
    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - 3 - gmstate.program_lastmove
    );
}

/// 测试用户移动操作失败
#[test]
fn user_move_failed() {
    let sys = System::new();
    init_game(&sys, 5, 2); // 初始化游戏
    let game = sys.get_program(1).unwrap(); // 获取游戏实例

    // 用户尝试无效的移动操作
    let res = game.send(USERS[0], PebblesAction::Turn(0)); // 移动弹珠数为0
    assert!(res.main_failed()); // 确保操作失败

    let res = game.send(USERS[0], PebblesAction::Turn(3)); // 移动弹珠数超出最大值
    assert!(res.main_failed()); // 确保操作失败
}

/// 测试用户移动操作失败情况2
#[test]
fn user_move_failed2() {
    let sys2 = System::new();
    init_game(&sys2, 3, 2); // 初始化游戏

    let game = sys2.get_program(1).unwrap(); // 获取游戏实例
    loop {
        let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。"); // 读取游戏状态
        if gmstate.program_lastmove == 2 {
            break; // 如果程序上一次移动数为2，则退出循环
        }
        game.send(
            USERS[0],
            PebblesAction::Restart {
                difficulty: DifficultyLevel::Easy,
                pebbles_count: 3,
                max_pebbles_per_turn: 2,
            },
        );
    }
    let res = game.send(USERS[0], PebblesAction::Turn(2)); // 移动弹珠数为2
    assert!(res.main_failed()); // 确保操作失败
}

/// 测试程序移动操作
#[test]
fn program_move() {
    let sys = System::new();
    init_game(&sys, 99, 3); // 初始化游戏
    let game = sys.get_program(1).unwrap(); // 获取游戏实例
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。"); // 读取游戏状态
    let mut remaing = gmstate.pebbles_remaining; // 获取剩余弹珠数

    // 测试程序移动操作
    let res = game.send(USERS[0], PebblesAction::GiveUp);
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。");
    assert!(res.contains(&(
        USERS[0],
        PebblesEvent::CounterTurn(gmstate.program_lastmove).encode()
    )));
    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - gmstate.program_lastmove
    );

    remaing = gmstate.pebbles_remaining;
    let res = game.send(USERS[0], PebblesAction::GiveUp);
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。");
    assert!(res.contains(&(
        USERS[0],
        PebblesEvent::CounterTurn(gmstate.program_lastmove).encode()
    )));
    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - gmstate.program_lastmove
    );

    remaing = gmstate.pebbles_remaining;
    let res = game.send(USERS[0], PebblesAction::GiveUp);
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。");
    assert!(res.contains(&(
        USERS[0],
        PebblesEvent::CounterTurn(gmstate.program_lastmove).encode()
    )));
    assert_eq!(
        gmstate.pebbles_remaining,
        remaing - gmstate.program_lastmove
    );
}

/// 测试游戏胜利逻辑
#[test]
fn winner() {
    let sys = System::new();
    init_game(&sys, 3, 1); // 初始化游戏
    let game = sys.get_program(1).unwrap(); // 获取游戏实例

    for _ in 0..100 {
        // 重启游戏
        game.send(
            USERS[0],
            PebblesAction::Restart {
                difficulty: DifficultyLevel::Easy,
                pebbles_count: 3,
                max_pebbles_per_turn: 1,
            },
        );
        let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。"); // 读取游戏状态
        let remaing = gmstate.pebbles_remaining; // 获取剩余弹珠数
        if remaing < 3 {
            let res = game.send(USERS[0], PebblesAction::Turn(1)); // 用户进行移动操作
            assert!(res.contains(&(USERS[0], PebblesEvent::Won(Player::Program).encode())));
        // 确保程序获胜事件被触发
        } else {
            let res = game.send(USERS[0], PebblesAction::Turn(1)); // 用户进行移动操作
            assert!(res.contains(&(USERS[0], PebblesEvent::CounterTurn(1).encode()))); // 确保计数器回合事件被触发
            let res = game.send(USERS[0], PebblesAction::Turn(1)); // 用户进行移动操作
            assert!(res.contains(&(USERS[0], PebblesEvent::Won(Player::User).encode())));
            // 确保用户获胜事件被触发
        }
    }
}

/// 测试重启游戏
#[test]
fn restart() {
    let sys = System::new();
    init_game(&sys, 3, 1); // 初始化游戏
    let game = sys.get_program(1).unwrap(); // 获取游戏实例
    let res = game.send(
        USERS[0],
        PebblesAction::Restart {
            difficulty: DifficultyLevel::Easy,
            pebbles_count: 50,       // 设置新的弹珠总数
            max_pebbles_per_turn: 3, // 设置新的每轮最大弹珠数
        },
    );
    assert!(!res.main_failed()); // 确保重启操作没有失败
    let gmstate: PebbleGame = game.read_state(0).expect("无效的状态。"); // 读取游戏状态
    assert_eq!(gmstate.pebbles_count, 50); // 检查新的弹珠总数
    assert_eq!(gmstate.max_pebbles_per_turn, 3); // 检查新的每轮最大弹珠数
}
