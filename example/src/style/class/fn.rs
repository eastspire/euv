use super::*;

class! {
    pub c_game_stats_bar {
        display: "flex";
        gap: "16px";
        margin-bottom: "12px";
        font-size: "14px";
        font-weight: "600";
    }

    pub c_game_stats_label {
        color: "inherit";
    }

    pub c_game_stats_fps_value {
        color: "inherit";
    }

    pub c_game_stats_count_value {
        color: "inherit";
    }

    pub c_game_stats_total_value {
        color: "inherit";
    }

    pub c_game_description {
        line-height: "1.5";
        color: "inherit";
        margin-bottom: var!(gap-component);
    }

    pub c_game_canvas_wrapper {
        position: "relative";
        width: "100%";
    }

    pub c_game_fullscreen_canvas_wrapper {
        flex: "1";
        display: "flex";
        align-items: "center";
        justify-content: "center";
        overflow: "hidden";
        min-height: "0";
        // The canvas inside has aspect-ratio: 3/2 and is allowed to
        // shrink to fit the column. This wrapper fills the column
        // (1248x750 on a 1280x800 viewport) so the canvas's max-width
        // and max-height are constrained by it, giving the largest
        // 3:2 landscape canvas that fits without overflow.
        width: "100%";
        height: "100%";
    }

    pub c_game_3d_canvas {
        width: "100%";
        height: "100%";
        cursor: "grab";
        display: "block";
        background: var!(accent);
        touch-action: "none";
        object-fit: "contain";
    }

    pub c_game_3d_canvas_fullscreen {
        // Landscape 3:2 aspect (same ratio as the inline canvas and the
        // 600x400 backing buffer). `height: 100%; width: auto` plus
        // `max-width: 100%` makes the canvas take the largest 3:2
        // landscape box that fits inside its flex container, which is
        // what maximises width utilisation in fullscreen mode.
        height: "100%";
        width: "auto";
        max-width: "100%";
        max-height: "100%";
        cursor: "grab";
        display: "block";
        background: var!(accent);
        touch-action: "none";
        object-fit: "contain";
        aspect-ratio: "3 / 2";
    }

    pub c_game_2d_canvas {
        width: "100%";
        height: "100%";
        cursor: "pointer";
        display: "block";
        background: var!(accent);
        touch-action: "none";
        object-fit: "contain";
    }

    pub c_game_2d_canvas_fullscreen {
        // Landscape 3:2 aspect (same ratio as the inline canvas and the
        // 600x400 backing buffer). See c_game_3d_canvas_fullscreen.
        height: "100%";
        width: "auto";
        max-width: "100%";
        max-height: "100%";
        cursor: "pointer";
        display: "block";
        background: var!(accent);
        touch-action: "none";
        object-fit: "contain";
        aspect-ratio: "3 / 2";
    }

    pub c_canvas_pixelated {
        image-rendering: "pixelated";
        image-rendering: "crisp-edges";
    }

    pub c_game_loading_overlay {
        position: "absolute";
        top: "0";
        left: "0";
        width: "100%";
        height: "100%";
        pointer-events: "none";
    }

    pub c_keep_alive_tab_visible {
        display: "block";
    }

    pub c_keep_alive_tab_hidden {
        display: "none";
    }

    pub c_binding_slider_label_accent {
        color: var!(accent);
    }

    pub c_binding_color_preview_bg(background: &str) {
        background: {
            background
        };
    }

    pub c_slider_value(value_percent: &str) {
        {
            "--value"
        }
        : {
            value_percent
        };
    }

    pub c_anim_scale_shrink {
        transform: "scale(0.85)";
    }

    pub c_anim_scale_normal {
        transform: "scale(1)";
    }
}
