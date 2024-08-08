#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::prelude::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

/// 处理游戏元数据的结构体
pub struct PebblesMetadata;

impl Metadata for PebblesMetadata {
    type Init = In<PebblesInit>; // 初始化参数
    type Handle = InOut<PebblesAction, PebblesEvent>; // 处理操作和事件
    type State = Out<GameState>; // 游戏状态
    type Reply = (); // 无回复
    type Others = (); // 其他
    type Signal = (); // 信号
}

/// 游戏初始化的配置结构体
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel, // 难度等级
    pub pebbles_count: u32,          // 弹珠总数
    pub max_pebbles_per_turn: u32,   // 每轮最大弹珠数
}

/// 游戏难度等级枚举
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq)]
pub enum DifficultyLevel {
    #[default]
    Easy, // 简单模式
    Hard, // 难模式
}

/// 游戏操作枚举
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    Turn(u32), // 用户移动操作
    GiveUp,    // 用户放弃操作
    Restart {
        // 重启游戏
        difficulty: DifficultyLevel, // 难度等级
        pebbles_count: u32,          // 弹珠总数
        max_pebbles_per_turn: u32,   // 每轮最大弹珠数
    },
}

/// 游戏事件枚举
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesEvent {
    CounterTurn(u32), // 程序移动事件
    Won(Player),      // 胜利事件
}

/// 玩家枚举
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum Player {
    #[default]
    User, // 用户
    Program, // 程序
}

/// 游戏状态结构体
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u32,          // 弹珠总数
    pub max_pebbles_per_turn: u32,   // 每轮最大弹珠数
    pub pebbles_remaining: u32,      // 剩余弹珠数
    pub difficulty: DifficultyLevel, // 难度等级
    pub first_player: Player,        // 首先移动的玩家
    pub winner: Option<Player>,      // 胜利者
}
