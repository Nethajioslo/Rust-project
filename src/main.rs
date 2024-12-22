/**
 ******************************************************************************
 * @file    main.rs
 * @author  Converted from C by Assistant
 * @version V1.0
 * @brief   Default main function.
 ******************************************************************************
 */
mod led;
mod syscalls;
use syscalls::initialise_monitor_handles;
use stm32_startup;

use crate::led::{led_init_all, led_on, led_off, LED_GREEN, LED_ORANGE, LED_BLUE, LED_RED};

use core::ptr::{read_volatile, write_volatile};

const MAX_TASKS: usize = 5;
const TICK_HZ: u32 = 1000;
const SYSTICK_TIM_CLK: u32 = 16_000_000;
const DUMMY_XPSR: u32 = 0x01000000;

const TASK_READY_STATE: u8 = 0;
const TASK_BLOCKED_STATE: u8 = 1;

const SCHED_STACK_START: u32 = 0x20001000;
const IDLE_STACK_START: u32 = 0x20001500;
const T1_STACK_START: u32 = 0x20002000;
const T2_STACK_START: u32 = 0x20002500;
const T3_STACK_START: u32 = 0x20003000;
const T4_STACK_START: u32 = 0x20003500;

static mut CURRENT_TASK: u8 = 1;
static mut G_TICK_COUNT: u32 = 0;

const CONST_V_1: u32 = 100;
const CONST_V_2: u32 = 100;
const CONST_V_3: u8 = 100;

#[derive(Copy, Clone)]
struct TCB {
    psp_value: u32,
    block_count: u32,
    current_state: u8,
    task_handler: fn(),
}

static mut USER_TASKS: [TCB; MAX_TASKS] = [TCB {
    psp_value: 0,
    block_count: 0,
    current_state: 0,
    task_handler: idle_task,
}; MAX_TASKS];

fn main() {
    enable_processor_faults();
    initialise_monitor_handles();
    init_scheduler_stack(SCHED_STACK_START);
    
    println!("Implementation of simple task scheduler");
    
    init_tasks_stack();
    led_init_all();
    init_systick_timer(TICK_HZ);
    switch_sp_to_psp();
    task1_handler();
    
    loop {}
}

fn idle_task() {
    loop {}
}

fn task1_handler() {
    loop {
        println!("Task1 is executing");
        led_on(LED_GREEN);
        task_delay(1000);
        led_off(LED_GREEN);
        task_delay(1000);
    }
}

fn task2_handler() {
    loop {
        println!("Task2 is executing");
        led_on(LED_ORANGE);
        task_delay(1000);
        led_off(LED_ORANGE);
        task_delay(1000);
    }
}

fn task3_handler() {
    loop {
        println!("Task3 is executing");
        led_on(LED_BLUE);
        task_delay(250);
        led_off(LED_BLUE);
        task_delay(250);
    }
}

fn task4_handler() {
    loop {
        println!("Task4 is executing");
        led_on(LED_RED);
        task_delay(125);
        led_off(LED_RED);
        task_delay(125);
    }
}

fn init_systick_timer(tick_hz: u32) {
    let srvr = 0xE000E014 as *mut u32;
    let scsr = 0xE000E010 as *mut u32;
    
    let count_value = (SYSTICK_TIM_CLK / tick_hz) - 1;
    
    unsafe {
        write_volatile(srvr, read_volatile(srvr) & !0x00FFFFFF);
        write_volatile(srvr, read_volatile(srvr) | count_value);
        write_volatile(scsr, read_volatile(scsr) | (1 << 1));
        write_volatile(scsr, read_volatile(scsr) | (1 << 2));
        write_volatile(scsr, read_volatile(scsr) | (1 << 0));
    }
}

fn init_tasks_stack() {
    unsafe {
        for i in 0..MAX_TASKS {
            USER_TASKS[i].current_state = TASK_READY_STATE;
        }
        
        USER_TASKS[0].psp_value = IDLE_STACK_START;
        USER_TASKS[1].psp_value = T1_STACK_START;
        USER_TASKS[2].psp_value = T2_STACK_START;
        USER_TASKS[3].psp_value = T3_STACK_START;
        USER_TASKS[4].psp_value = T4_STACK_START;
        
        USER_TASKS[0].task_handler = idle_task;
        USER_TASKS[1].task_handler = task1_handler;
        USER_TASKS[2].task_handler = task2_handler;
        USER_TASKS[3].task_handler = task3_handler;
        USER_TASKS[4].task_handler = task4_handler;
        
        for i in 0..MAX_TASKS {
            let mut psp = USER_TASKS[i].psp_value;
            
            psp -= 4;
            *(psp as *mut u32) = DUMMY_XPSR;
            
            psp -= 4;
            *(psp as *mut u32) = USER_TASKS[i].task_handler as u32;
            
            psp -= 4;
            *(psp as *mut u32) = 0xFFFFFFFD;
            
            for _ in 0..13 {
                psp -= 4;
                *(psp as *mut u32) = 0;
            }
            
            USER_TASKS[i].psp_value = psp;
        }
    }
}

fn enable_processor_faults() {
    let shcsr = 0xE000ED24 as *mut u32;
    unsafe {
        write_volatile(shcsr, read_volatile(shcsr) | (1 << 16));
        write_volatile(shcsr, read_volatile(shcsr) | (1 << 17));
        write_volatile(shcsr, read_volatile(shcsr) | (1 << 18));
    }
}

fn get_psp_value() -> u32 {
    unsafe { USER_TASKS[CURRENT_TASK as usize].psp_value }
}

fn save_psp_value(current_psp_value: u32) {
    unsafe {
        USER_TASKS[CURRENT_TASK as usize].psp_value = current_psp_value;
    }
}

fn update_next_task() {
    unsafe {
        let mut state = TASK_BLOCKED_STATE;
        
        for _ in 0..MAX_TASKS {
            CURRENT_TASK += 1;
            CURRENT_TASK %= MAX_TASKS as u8;
            state = USER_TASKS[CURRENT_TASK as usize].current_state;
            if state == TASK_READY_STATE && CURRENT_TASK != 0 {
                break;
            }
        }
        
        if state != TASK_READY_STATE {
            CURRENT_TASK = 0;
        }
    }
}

fn task_delay(tick_count: u32) {
    disable_interrupts();
    
    unsafe {
        if CURRENT_TASK != 0 {
            USER_TASKS[CURRENT_TASK as usize].block_count = G_TICK_COUNT + tick_count;
            USER_TASKS[CURRENT_TASK as usize].current_state = TASK_BLOCKED_STATE;
            schedule();
        }
    }
    
    enable_interrupts();
}

fn update_global_tick_count() {
    unsafe {
        G_TICK_COUNT += 1;
    }
}

fn unblock_tasks() {
    unsafe {
        for i in 1..MAX_TASKS {
            if USER_TASKS[i].current_state != TASK_READY_STATE {
                if USER_TASKS[i].block_count == G_TICK_COUNT {
                    USER_TASKS[i].current_state = TASK_READY_STATE;
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn SysTick_Handler() {
    let icsr = 0xE000ED04 as *mut u32;
    
    update_global_tick_count();
    unblock_tasks();
    
    unsafe {
        write_volatile(icsr, read_volatile(icsr) | (1 << 28));
    }
}

#[no_mangle]
pub extern "C" fn HardFault_Handler() {
    println!("Exception : Hardfault");
    loop {}
}

#[no_mangle]
pub extern "C" fn MemManage_Handler() {
    println!("Exception : MemManage");
    loop {}
}

#[no_mangle]
pub extern "C" fn BusFault_Handler() {
    println!("Exception : BusFault");
    loop {}
}
