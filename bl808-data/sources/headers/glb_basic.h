struct glb_reg {
    /* 0x0 : soc_info0 */
    union {
        struct {
            uint32_t reserved_0_26 : 27; /* [26: 0],       rsvd,        0x0 */
            uint32_t chip_rdy      : 1;  /* [   27],          r,        0x0 */
            uint32_t glb_id        : 4;  /* [31:28],          r,        0x6 */
        } BF;
        uint32_t WORD;
    } soc_info0;

    /* 0x4  reserved */
    uint8_t RESERVED0x4[76];

    /* 0x50 : core_cfg16 */
    union {
        struct {
            uint32_t np_int_sta0 : 32; /* [31: 0],          r,        0x0 */
        } BF;
        uint32_t WORD;
    } core_cfg16;

    /* 0x8C4 : gpio_cfg0 */
    union {
        struct {
            uint32_t reg_gpio_0_ie           : 1; /* [    0],        r/w,        0x0 */
            uint32_t reg_gpio_0_smt          : 1; /* [    1],        r/w,        0x1 */
            uint32_t reg_gpio_0_drv          : 2; /* [ 3: 2],        r/w,        0x0 */
            uint32_t reg_gpio_0_pu           : 1; /* [    4],        r/w,        0x0 */
            uint32_t reg_gpio_0_pd           : 1; /* [    5],        r/w,        0x0 */
            uint32_t reg_gpio_0_oe           : 1; /* [    6],        r/w,        0x0 */
            uint32_t reserved_7              : 1; /* [    7],       rsvd,        0x0 */
            uint32_t reg_gpio_0_func_sel     : 5; /* [12: 8],        r/w,        0xb */
            uint32_t reserved_13_15          : 3; /* [15:13],       rsvd,        0x0 */
            uint32_t reg_gpio_0_int_mode_set : 4; /* [19:16],        r/w,        0x0 */
            uint32_t reg_gpio_0_int_clr      : 1; /* [   20],        r/w,        0x0 */
            uint32_t gpio_0_int_stat         : 1; /* [   21],          r,        0x0 */
            uint32_t reg_gpio_0_int_mask     : 1; /* [   22],        r/w,        0x1 */
            uint32_t reserved_23             : 1; /* [   23],       rsvd,        0x0 */
            uint32_t reg_gpio_0_o            : 1; /* [   24],        r/w,        0x0 */
            uint32_t reg_gpio_0_set          : 1; /* [   25],        w1p,        0x0 */
            uint32_t reg_gpio_0_clr          : 1; /* [   26],        w1p,        0x0 */
            uint32_t reserved_27             : 1; /* [   27],       rsvd,        0x0 */
            uint32_t reg_gpio_0_i            : 1; /* [   28],          r,        0x0 */
            uint32_t reserved_29             : 1; /* [   29],       rsvd,        0x0 */
            uint32_t reg_gpio_0_mode         : 2; /* [31:30],        r/w,        0x0 */
        } BF;
        uint32_t WORD;
    } gpio_cfg0;

    /* 0x900 : gpio_cfg15 */
    union {
        struct {
            uint32_t reg_gpio_15_ie           : 1; /* [    0],        r/w,        0x0 */
            uint32_t reg_gpio_15_smt          : 1; /* [    1],        r/w,        0x1 */
            uint32_t reg_gpio_15_drv          : 2; /* [ 3: 2],        r/w,        0x0 */
            uint32_t reg_gpio_15_pu           : 1; /* [    4],        r/w,        0x0 */
            uint32_t reg_gpio_15_pd           : 1; /* [    5],        r/w,        0x0 */
            uint32_t reg_gpio_15_oe           : 1; /* [    6],        r/w,        0x0 */
            uint32_t reserved_7               : 1; /* [    7],       rsvd,        0x0 */
            uint32_t reg_gpio_15_func_sel     : 5; /* [12: 8],        r/w,        0xb */
            uint32_t reserved_13_15           : 3; /* [15:13],       rsvd,        0x0 */
            uint32_t reg_gpio_15_int_mode_set : 4; /* [19:16],        r/w,        0x0 */
            uint32_t reg_gpio_15_int_clr      : 1; /* [   20],        r/w,        0x0 */
            uint32_t gpio_15_int_stat         : 1; /* [   21],          r,        0x0 */
            uint32_t reg_gpio_15_int_mask     : 1; /* [   22],        r/w,        0x1 */
            uint32_t reserved_23              : 1; /* [   23],       rsvd,        0x0 */
            uint32_t reg_gpio_15_o            : 1; /* [   24],        r/w,        0x0 */
            uint32_t reg_gpio_15_set          : 1; /* [   25],        w1p,        0x0 */
            uint32_t reg_gpio_15_clr          : 1; /* [   26],        w1p,        0x0 */
            uint32_t reserved_27              : 1; /* [   27],       rsvd,        0x0 */
            uint32_t reg_gpio_15_i            : 1; /* [   28],          r,        0x0 */
            uint32_t reserved_29              : 1; /* [   29],       rsvd,        0x0 */
            uint32_t reg_gpio_15_mode         : 2; /* [31:30],        r/w,        0x0 */
        } BF;
        uint32_t WORD;
    } gpio_cfg15;
};

typedef volatile struct glb_reg glb_reg_t;
