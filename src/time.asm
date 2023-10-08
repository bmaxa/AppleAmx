.text
.global _init_time
.global _time_me
_init_time:
    mrs x0,CNTPCT_EL0 ; counter
    ret
_time_me:
    mrs x8,cntfrq_el0 ; clock
    ucvtf d1,x8
    mrs x8,CNTPCT_EL0 ; counter
    sub x8,x8,x0
    ucvtf d0,x8
    fdiv d0,d0,d1
    ret
