package client

import t "../vendor/TermCL/"
import "../vendor/TermCL/term/"

main :: proc()
{
    scr := t.init_screen(term.VTABLE);
    defer t.destroy_screen(&scr);

    t.clear(&scr, .Everything);
    t.write(&scr, "Hello, World!");
    t.blit(&scr);
}
