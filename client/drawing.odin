package client

import t "../vendor/TermCL/"

RUNE_H_LINE :: 0x2500 // ─
RUNE_V_LINE :: 0x2502 // │
RUNE_TL     :: 0x250C // ┌
RUNE_TR     :: 0x2510 // ┐
RUNE_BL     :: 0x2514 // └
RUNE_BR     :: 0x2518 // ┘

draw_hline :: proc(scr: ^t.Screen, x: uint, y: uint, len: uint) {
    for i := uint(0); i < len; i += 1 {
        t.move_cursor(scr, y, x + i)
        t.write_rune(scr, RUNE_H_LINE)
    }
}

draw_vline :: proc(scr: ^t.Screen, x: uint, y: uint, len: uint) {
    for i := uint(0); i < len; i += 1 {
        t.move_cursor(scr, y + i, x)
        t.write_rune(scr, RUNE_V_LINE)
    }
}

draw_box :: proc(scr: ^t.Screen, x: uint, y: uint, w: uint, h: uint) {
    // 1. Draw the four sides
    draw_hline(scr, x, y, w)             
    draw_hline(scr, x, y + h - 1, w)     
    draw_vline(scr, x, y, h)             
    draw_vline(scr, x + w - 1, y, h)     

    // 2. Overwrite the corners (Top-Left, Top-Right, Bottom-Left, Bottom-Right)
    t.move_cursor(scr, y, x)
    t.write_rune(scr, RUNE_TL)

    t.move_cursor(scr, y, x + w - 1)
    t.write_rune(scr, RUNE_TR)

    t.move_cursor(scr, y + h - 1, x)
    t.write_rune(scr, RUNE_BL)

    t.move_cursor(scr, y + h - 1, x + w - 1)
    t.write_rune(scr, RUNE_BR)
}
