/// Raw stylesheet for rendered markdown bodies ([`crate::euv_markdown`]).
///
/// `class!` cannot express descendant selectors, so markdown typography is
/// injected once at startup through `Css::inject_css`. All colors reference
/// the theme CSS variables, so light/dark switching works unchanged.
pub const EUV_MD_CSS: &str = r#"
.md-body {
    line-height: 1.7;
    font-size: var(--font-base);
    word-wrap: break-word;
}
.md-body h1, .md-body h2, .md-body h3, .md-body h4, .md-body h5, .md-body h6 {
    position: relative;
    font-weight: 700;
    letter-spacing: -0.01em;
    margin-top: 1.8em;
    margin-bottom: 0.6em;
    scroll-margin-top: 72px;
    line-height: 1.3;
}
.md-body h1 {
    font-size: var(--font-3xl);
    margin-top: 0;
    padding-bottom: 0.4em;
    border-bottom: 1px solid var(--border);
}
.md-body h2 {
    font-size: var(--font-2xl);
    padding-bottom: 0.3em;
    border-bottom: 1px dashed var(--border);
}
.md-body h3 { font-size: var(--font-xl); }
.md-body h4 { font-size: var(--font-lg); }
.md-body h5, .md-body h6 { font-size: var(--font-base); }
.md-body .header-anchor {
    float: left;
    margin-left: -0.9em;
    padding-right: 0.2em;
    opacity: 0;
    color: var(--muted-foreground);
    font-weight: 400;
    transition: opacity 0.15s ease-out;
    user-select: none;
    /* The `.md-body a` rule below applies `text-decoration: underline
       dashed` to every anchor. The header-anchor is a typographic
       icon, not a navigable link in the visual sense, so drop the
       underline. */
    text-decoration: none;
}
.md-body h1:hover .header-anchor,
.md-body h2:hover .header-anchor,
.md-body h3:hover .header-anchor,
.md-body h4:hover .header-anchor,
.md-body h5:hover .header-anchor,
.md-body h6:hover .header-anchor {
    opacity: 1;
}
/* On narrow viewports the negative `margin-left: -0.9em` pushes the `#`
   sign past the left edge of the viewport (anchor x ≈ -11px on a 380px
   viewport), so hover-revealed anchors get clipped. On mobile we drop
   the float + negative margin and absolutely position the anchor at
   the heading's left edge instead, with the heading content shifted
   right via `padding-left`. This way:

   * The `#` stays inside the viewport at all times.
   * The `#` baseline aligns with the heading's first line baseline
     (`vertical-align: middle` on a single-line-height inline-flex
     box puts the `#` on the same horizontal line as the heading
     text, not against the heading's overall vertical centre).
   * The heading text gets the full content width (minus the small
     reserved gutter), so a long heading like "Markdown Features" no
     longer wraps with one orphan word on its own line leaving a big
     empty space at the end of the first line.

   Earlier revisions used `height: 100%; align-items: center`, which
   stretched the anchor box to the full heading height and put the
   `#` on the heading's geometric centre. For a wrapped h1 like
   "Markdown Features" that ended up aligned with the second line,
   not the first — visually it looked like the `#` belonged to
   "Features" rather than "Markdown". */
@media (max-width: 767px) {
    /* On narrow viewports the negative `margin-left: -0.9em` on the
       anchor pushes the `#` sign past the left edge of the viewport
       (anchor x ≈ -11px on a 380px viewport), so hover-revealed
       anchors get clipped. We rework the mobile anchor to live
       inside the heading's inline flow instead of floating to the
       left:

       * The heading itself gets a `padding-left` so its inline
         content (the actual heading text) still starts at the same
         visual x as before.
       * The anchor is the FIRST inline child of the heading, with
         a `margin-left: -1.6em` so it pulls itself into the gutter
         reserved by that padding.
       * With `vertical-align: baseline` and a single-line-height
         box (`line-height: 1`), the `#` glyph baseline lines up
         with the heading text baseline on the FIRST line — so the
         `#` and "Markdown" share the same horizontal line, instead
         of the `#` being vertically centred against the whole
         heading box (which, for wrapped headings like "Markdown
         Features", used to put `#` on the line between the two
         words).
       * The heading text gets the full content width (minus the
         gutter), so a long heading like "Markdown Features" no
         longer wraps with one orphan word on its own line leaving
         a big empty space at the end of the first line — because
         the anchor no longer occupies any inline space on the
         first line. */
    .md-body h1,
    .md-body h2,
    .md-body h3,
    .md-body h4,
    .md-body h5,
    .md-body h6 {
        padding-left: 1.6em;
    }
    .md-body .header-anchor {
        float: none;
        margin-left: -1.6em;
        margin-right: 0;
        padding: 0;
        width: 1.6em;
        display: inline-flex;
        align-items: flex-end;
        justify-content: flex-start;
        font-size: 0.85em;
        line-height: 1;
        vertical-align: baseline;
    }
}
.md-body p, .md-body ul, .md-body ol, .md-body blockquote, .md-body pre, .md-body table {
    margin: 1em 0;
}
.md-body ul, .md-body ol {
    padding-left: 1.4em;
}
.md-body ul { list-style: disc; }
.md-body ol { list-style: decimal; }
.md-body ul ul, .md-body ul ol, .md-body ol ul, .md-body ol ol {
    margin: 0.25em 0;
}
.md-body li { margin: 0.25em 0; }
.md-body li input[type="checkbox"] {
    margin-right: 0.4em;
    accent-color: var(--accent);
}
.md-body a {
    color: var(--accent);
    font-weight: 500;
    text-decoration: underline;
    text-underline-offset: 3px;
    text-decoration-style: dashed;
    text-decoration-color: var(--border);
}
.md-body a:hover {
    text-decoration-style: solid;
    text-decoration-color: var(--accent);
}
.md-body strong { font-weight: 700; }
.md-body em { font-style: italic; }
.md-body del { opacity: 0.6; }
.md-body hr {
    border: none;
    border-top: 1px dashed var(--border);
    margin: 2em 0;
}
.md-body blockquote {
    margin: 1em 0;
    padding: 0.4em 1em;
    border-left: 4px solid var(--border);
    color: var(--muted-foreground);
}
.md-body blockquote p { margin: 0.4em 0; }
.md-body code {
    font-family: ui-monospace, monospace;
    font-size: 0.875em;
    padding: 0.15em 0.4em;
    background: var(--accent-muted);
    border: 1px solid var(--border);
    /* Inline <code> that wraps to multiple lines must keep its border
       on every fragment, otherwise the first line loses its right border
       and subsequent lines lose their left border.

       box-decoration-break: clone alone is not enough in practice: the
       per-fragment borders stack against the next fragment's background
       and the visual gap between fragments collapses, making the right
       border of one line look like the left border of the next (and
       neither looks fully closed). Making the element inline-block
       guarantees each fragment draws its own box with its own four
       borders, exactly the same trick used by euv_tag. */
    -webkit-box-decoration-break: clone;
    box-decoration-break: clone;
    display: inline-block;
    /* `vertical-align: baseline` plus `line-height: 1` keeps the
       <code> box visually on the same baseline as the surrounding
       text. Earlier revisions used `vertical-align: text-top` with
       `line-height: 1.4`, which pushed the box downward and made the
       framed text look like it was sitting on a lower line than the
       surrounding body text — a small offset, but very noticeable in
       tight running prose like list items and paragraphs. With
       `line-height: 1` the inline-block has no extra leading so its
       baseline aligns with the baseline of the parent line. */
    vertical-align: baseline;
    line-height: 1;
}
/* In a table cell an inline-block <code> collapses to the width of
   its longest word plus padding, leaving a tall narrow box with large
   blank gaps on every other line. Promote it to `display: block` so
   the box fills the cell and the text wraps naturally, while still
   preserving the per-fragment border from the rule above.

   We do NOT use `td > code` here. euv-docs / VuePress insert
   `display: contents` <div> / <slot> wrappers between the cell and
   the <code> element at runtime, so the direct-child selector
   never matches in the deployed build. Using `td code` instead
   reaches the same <code> regardless of those transparent wrappers.

   Specificity check: `.md-body td code` is (0,1,2) and the base
   `.md-body code` rule is (0,1,1), so this rule wins without
   needing `!important` or repeated `.md-body` classes. */
.md-body td code,
.md-body th code {
    display: block;
}
.md-body pre {
    padding: 1em 1.2em;
    overflow-x: auto;
    border: 1px solid var(--border);
    background: var(--accent-muted);
}
.md-body pre code {
    padding: 0;
    border: none;
    background: transparent;
    font-size: 0.875rem;
    line-height: 1.6;
}
.md-body table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-sm);
    display: block;
    overflow-x: auto;
}
.md-body table thead { border-bottom: 2px solid var(--border); }
.md-body table th, .md-body table td {
    padding: 0.5em 0.9em;
    border: 1px solid var(--border);
    text-align: left;
}
.md-body table th { font-weight: 700; }
.md-body table tbody tr:nth-child(2n) { background: var(--accent-muted); }
.md-body img { max-width: 100%; }
.md-body .docs-container {
    margin: 1.2em 0;
    padding: 0.1em 1.2em;
    border-left: 4px solid var(--foreground);
    background: var(--accent-muted);
}
.md-body .docs-container .docs-container-title {
    font-weight: 700;
    font-size: var(--font-sm);
    letter-spacing: 0.04em;
    text-transform: uppercase;
    margin: 0.8em 0 0.4em;
}
.md-body .docs-container.tip { border-left-style: solid; }
.md-body .docs-container.warning { border-left-style: dashed; }
.md-body .docs-container.danger {
    border-left: 4px double var(--foreground);
}
.md-body .docs-container.details {
    border-left: 1px solid var(--border);
    background: transparent;
}
.md-body .footnote-definition { font-size: var(--font-sm); color: var(--muted-foreground); }
"#;
