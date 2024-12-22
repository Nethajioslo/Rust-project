
const SRAM_START: u32 = 0x2000_0000;
const SRAM_SIZE: u32 = 128 * 1024; // 128KB
const SRAM_END: u32 = SRAM_START + SRAM_SIZE;
const STACK_START: u32 = SRAM_END;

extern "C" {
    static _etext: u32;
    static _sdata: u32;
    static _edata: u32;
    static _la_data: u32;
    static _sbss: u32;
    static _ebss: u32;
    
    fn main() -> i32;
    fn __libc_init_array();
}

#[no_mangle]
pub extern "C" fn Reset_Handler() {
    let size = unsafe { (&_edata as *const u32 as usize) - (&_sdata as *const u32 as usize) };//sram
    let p_dst = unsafe { &_sdata as *const u32 as *mut u8 };//flash
    let p_src = unsafe { &_la_data as *const u32 as *const u8 };

    for i in 0..size {
        unsafe {
            *p_dst.add(i) = *p_src.add(i);
        }
    }

    let size = unsafe { (&_ebss as *const u32 as usize) - (&_sbss as *const u32 as usize) };
    let p_dst = unsafe { &_sbss as *const u32 as *mut u8 };
    
    for i in 0..size {
        unsafe {
            *p_dst.add(i) = 0;
        }
    }

    unsafe {
        __libc_init_array();
        main();
    }

    loop {}
}

/// Default handler for interrupts
#[no_mangle]
pub extern "C" fn Default_Handler() {
    loop {}
}

// Macro to generate interrupt handlers
macro_rules! make_default_handler {
    ($handler:ident) => {
        #[no_mangle]
        pub extern "C" fn $handler() {
            Default_Handler();
        }
    };
}

make_default_handler!(NMI_Handler);
make_default_handler!(HardFault_Handler);
make_default_handler!(MemManage_Handler);
make_default_handler!(BusFault_Handler);
make_default_handler!(UsageFault_Handler);
make_default_handler!(SVC_Handler);
make_default_handler!(DebugMon_Handler);
make_default_handler!(PendSV_Handler);
make_default_handler!(SysTick_Handler);
make_default_handler!(WWDG_IRQHandler);
make_default_handler!(PVD_IRQHandler);
make_default_handler!(TAMP_STAMP_IRQHandler);
make_default_handler!(RTC_WKUP_IRQHandler);
make_default_handler!(RCC_IRQHandler);
make_default_handler!(EXTI0_IRQHandler);
make_default_handler!(EXTI1_IRQHandler);
make_default_handler!(EXTI2_IRQHandler);
make_default_handler!(EXTI3_IRQHandler);
make_default_handler!(EXTI4_IRQHandler);
make_default_handler!(DMA1_Stream0_IRQHandler);
make_default_handler!(DMA1_Stream1_IRQHandler);
make_default_handler!(DMA1_Stream2_IRQHandler);
make_default_handler!(DMA1_Stream3_IRQHandler);
make_default_handler!(DMA1_Stream4_IRQHandler);
make_default_handler!(DMA1_Stream5_IRQHandler);
make_default_handler!(DMA1_Stream6_IRQHandler);
make_default_handler!(ADC_IRQHandler);
make_default_handler!(CAN1_TX_IRQHandler);
make_default_handler!(CAN1_RX0_IRQHandler);
make_default_handler!(CAN1_RX1_IRQHandler);
make_default_handler!(CAN1_SCE_IRQHandler);
make_default_handler!(EXTI9_5_IRQHandler);
make_default_handler!(TIM1_BRK_TIM9_IRQHandler);
make_default_handler!(TIM1_UP_TIM10_IRQHandler);
make_default_handler!(TIM1_TRG_COM_TIM11_IRQHandler);
make_default_handler!(TIM1_CC_IRQHandler);
make_default_handler!(TIM2_IRQHandler);
make_default_handler!(TIM3_IRQHandler);
make_default_handler!(TIM4_IRQHandler);
make_default_handler!(I2C1_EV_IRQHandler);
make_default_handler!(I2C1_ER_IRQHandler);
make_default_handler!(I2C2_EV_IRQHandler);
make_default_handler!(I2C2_ER_IRQHandler);
make_default_handler!(SPI1_IRQHandler);
make_default_handler!(SPI2_IRQHandler);
make_default_handler!(USART1_IRQHandler);
make_default_handler!(USART2_IRQHandler);
make_default_handler!(USART3_IRQHandler);
make_default_handler!(EXTI15_10_IRQHandler);
make_default_handler!(RTC_Alarm_IRQHandler);
make_default_handler!(OTG_FS_WKUP_IRQHandler);
make_default_handler!(TIM8_BRK_TIM12_IRQHandler);
make_default_handler!(TIM8_UP_TIM13_IRQHandler);
make_default_handler!(TIM8_TRG_COM_TIM14_IRQHandler);
make_default_handler!(TIM8_CC_IRQHandler);
make_default_handler!(DMA1_Stream7_IRQHandler);
make_default_handler!(FSMC_IRQHandler);
make_default_handler!(SDIO_IRQHandler);
make_default_handler!(TIM5_IRQHandler);
make_default_handler!(SPI3_IRQHandler);
make_default_handler!(UART4_IRQHandler);
make_default_handler!(UART5_IRQHandler);
make_default_handler!(TIM6_DAC_IRQHandler);
make_default_handler!(TIM7_IRQHandler);
make_default_handler!(DMA2_Stream0_IRQHandler);
make_default_handler!(DMA2_Stream1_IRQHandler);
make_default_handler!(DMA2_Stream2_IRQHandler);
make_default_handler!(DMA2_Stream3_IRQHandler);
make_default_handler!(DMA2_Stream4_IRQHandler);
make_default_handler!(ETH_IRQHandler);
make_default_handler!(ETH_WKUP_IRQHandler);
make_default_handler!(CAN2_TX_IRQHandler);
make_default_handler!(CAN2_RX0_IRQHandler);
make_default_handler!(CAN2_RX1_IRQHandler);
make_default_handler!(CAN2_SCE_IRQHandler);
make_default_handler!(OTG_FS_IRQHandler);
make_default_handler!(DMA2_Stream5_IRQHandler);
make_default_handler!(DMA2_Stream6_IRQHandler);
make_default_handler!(DMA2_Stream7_IRQHandler);
make_default_handler!(USART6_IRQHandler);
make_default_handler!(I2C3_EV_IRQHandler);
make_default_handler!(I2C3_ER_IRQHandler);
make_default_handler!(OTG_HS_EP1_OUT_IRQHandler);
make_default_handler!(OTG_HS_EP1_IN_IRQHandler);
make_default_handler!(OTG_HS_WKUP_IRQHandler);
make_default_handler!(OTG_HS_IRQHandler);
make_default_handler!(DCMI_IRQHandler);
make_default_handler!(CRYP_IRQHandler);
make_default_handler!(HASH_RNG_IRQHandler);
make_default_handler!(FPU_IRQHandler);


#[link_section = ".isr_vector"]
#[no_mangle]
pub static VECTORS: [u32; 97] = [
    STACK_START,
    Reset_Handler as u32,
    NMI_Handler as u32,
    HardFault_Handler as u32,
    MemManage_Handler as u32,
    BusFault_Handler as u32,
    UsageFault_Handler as u32,
    0,
    0,
    0,
    0,
    SVC_Handler as u32,
    DebugMon_Handler as u32,
    0,
    PendSV_Handler,
    SysTick_Handler,
    WWDG_IRQHandler,
    PVD_IRQHandler,
    TAMP_STAMP_IRQHandler,
    RTC_WKUP_IRQHandler,
    0,
    RCC_IRQHandler as u32,
    EXTI0_IRQHandler as u32,
    EXTI1_IRQHandler as u32,
    EXTI2_IRQHandler as u32,
    EXTI3_IRQHandler as u32,
    EXTI4_IRQHandler as u32,
    DMA1_Stream0_IRQHandler as u32,
    DMA1_Stream1_IRQHandler as u32,
    DMA1_Stream2_IRQHandler as u32,
    DMA1_Stream3_IRQHandler as u32,
    DMA1_Stream4_IRQHandler as u32,
    DMA1_Stream5_IRQHandler as u32,
    DMA1_Stream6_IRQHandler as u32,
    ADC_IRQHandler as u32,
    CAN1_TX_IRQHandler as u32,
    CAN1_RX0_IRQHandler as u32,
    CAN1_RX1_IRQHandler as u32,
    CAN1_SCE_IRQHandler as u32,
    EXTI9_5_IRQHandler as u32,
    TIM1_BRK_TIM9_IRQHandler as u32,
    TIM1_UP_TIM10_IRQHandler as u32,
    TIM1_TRG_COM_TIM11_IRQHandler as u32,
    TIM1_CC_IRQHandler as u32,
    TIM2_IRQHandler as u32,
    TIM3_IRQHandler as u32,
    TIM4_IRQHandler as u32,
    I2C1_EV_IRQHandler as u32,
    I2C1_ER_IRQHandler as u32,
    I2C2_EV_IRQHandler as u32,
    I2C2_ER_IRQHandler as u32,
    SPI1_IRQHandler as u32,
    SPI2_IRQHandler as u32,
    USART1_IRQHandler as u32,
    USART2_IRQHandler as u32,
    USART3_IRQHandler as u32,
    EXTI15_10_IRQHandler as u32,
    RTC_Alarm_IRQHandler as u32,
    OTG_FS_WKUP_IRQHandler as u32,
    TIM8_BRK_TIM12_IRQHandler as u32,
    TIM8_UP_TIM13_IRQHandler as u32,
    TIM8_TRG_COM_TIM14_IRQHandler as u32,
    TIM8_CC_IRQHandler as u32,
    DMA1_Stream7_IRQHandler as u32,
    FSMC_IRQHandler as u32,
    SDIO_IRQHandler as u32,
    TIM5_IRQHandler as u32,
    SPI3_IRQHandler as u32,
    UART4_IRQHandler as u32,
    UART5_IRQHandler as u32,
    TIM6_DAC_IRQHandler as u32,
    TIM7_IRQHandler as u32,
    DMA2_Stream0_IRQHandler as u32,
    DMA2_Stream1_IRQHandler as u32,
    DMA2_Stream2_IRQHandler as u32,
    DMA2_Stream3_IRQHandler as u32,
    DMA2_Stream4_IRQHandler as u32,
    ETH_IRQHandler as u32,
    ETH_WKUP_IRQHandler as u32,
    CAN2_TX_IRQHandler as u32,
    CAN2_RX0_IRQHandler as u32,
    CAN2_RX1_IRQHandler as u32,
    CAN2_SCE_IRQHandler as u32,
    OTG_FS_IRQHandler as u32,
    DMA2_Stream5_IRQHandler as u32,
    DMA2_Stream6_IRQHandler as u32,
    DMA2_Stream7_IRQHandler as u32,
    USART6_IRQHandler as u32,
    I2C3_EV_IRQHandler as u32,
    I2C3_ER_IRQHandler as u32,
    OTG_HS_EP1_OUT_IRQHandler as u32,
    OTG_HS_EP1_IN_IRQHandler as u32,
    OTG_HS_WKUP_IRQHandler as u32,
    OTG_HS_IRQHandler as u32,
    DCMI_IRQHandler as u32,
    CRYP_IRQHandler as u32,
    HASH_RNG_IRQHandler as u32,
    FPU_IRQHandler as u32,
];