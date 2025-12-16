package client

import t "../vendor/TermCL/"

draw_box :: proc(scr: ^t.Screen, x: uint, y: uint, w: uint, h: uint)
{
    for i := uint(0); i < w; i += 1
    {
        t.move_cursor(scr, y, x + i);
        t.write_rune(scr, 0x2500);
    }

    for i := uint(0); i < h; i += 1
    {
        t.move_cursor(scr, y + i, x);
        t.write_rune(scr, 0x2502);
    }

    for i := uint(0); i < w; i += 1
    {
        t.move_cursor(scr, h - 1, x + i);
        t.write_rune(scr, 0x2500);
    }

    for i := uint(0); i < h; i += 1
    {
        t.move_cursor(scr, h + i, w - 1);
        t.write_rune(scr, 0x2502);
    }

    t.move_cursor(scr, y, x)
    t.write_rune(scr, 0x250C)

    t.move_cursor(scr, y, x + w - 1)
    t.write_rune(scr, 0x2510)

    t.move_cursor(scr, y + h - 1, x)
    t.write_rune(scr, 0x2514)

    t.move_cursor(scr, y + h - 1, x + w - 1)
    t.write_rune(scr, 0x2518)
}
