package client

import t "../vendor/TermCL/"
import "../vendor/TermCL/term/"

main :: proc()
{
    scr := t.init_screen(term.VTABLE)
    defer t.destroy_screen(&scr)

    t.set_term_mode(&scr, .Cbreak);
    t.hide_cursor(true);
    t.clear(&scr, .Everything);
    t.blit(&scr);

    state := State {
        cli_state = .SYNC,
        connection_status = .OFFLINE
    }

    for {
        defer t.blit(&scr);

        // close cli
        input := t.read(&scr);
        kb_input, kb_ok := input.(t.Keyboard_Input)

        if kb_input.key == .Q
        {
            return;
        }

        t.move_cursor(&scr, 0, 0);
        t.clear(&scr, .Everything);

        dim := t.get_window_size(&scr)
        draw_box(&scr, 0, 0, dim.w, dim.h)

        t.move_cursor(&scr, 0, 1);
        t.write(&scr, "Do it Later - Task CLI");

        #partial switch state.cli_state
        {
            case .SETUP:
            {

            }
        }
    }
}
