# html/semantics/popovers/popover-overlay.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-overlay.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>popover: overlay</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/popover.html#the-popover-attribute">
<link rel="help" href="https://drafts.csswg.org/css-position-4/#overlay">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<dialog popover id="popover-show-dialog"></dialog>
<dialog popover id="popover-show-modal-dialog"></dialog>
<dialog popover id="popover-dialog"></dialog>
<div popover id="popover-div"></div>
<script>
  test(() => {
    const popover_show_dialog = document.getElementById("popover-show-dialog");
    assert_equals(getComputedStyle(popover_show_dialog).overlay, "none",
                  "Computed overlay");
    popover_show_dialog.show();
    assert_equals(getComputedStyle(popover_show_dialog).overlay, "none",
                  "Computed overlay after show()");
    popover_show_dialog.close();
  }, "dialog.show() should not put popover dialog in top layer");

  test(() => {
    const popover_show_modal_dialog = document.getElementById("popover-show-modal-dialog");
    assert_equals(getComputedStyle(popover_show_modal_dialog).overlay, "none",
                  "Computed overlay");
    popover_show_modal_dialog.showModal();
    assert_equals(getComputedStyle(popover_show_modal_dialog).overlay, "auto",
                  "Computed overlay after showModal()");
    popover_show_modal_dialog.close();
  }, "dialog.showModal() should put popover dialog in top layer");

  test(() => {
    const popover_dialog = document.getElementById("popover-dialog");
    assert_equals(getComputedStyle(popover_dialog).overlay, "none",
                  "Computed overlay");
    popover_dialog.showPopover();
    assert_equals(getComputedStyle(popover_dialog).overlay, "auto",
                  "Computed overlay after showPopover()");
    popover_dialog.hidePopover();
  }, "dialog.showPopover() should put popover dialog in top layer");

  test(() => {
    const popover_div = document.getElementById("popover-div");
    assert_equals(getComputedStyle(popover_div).overlay, "none",
                  "Computed overlay");
    popover_div.showPopover();
    assert_equals(getComputedStyle(popover_div).overlay, "auto",
                  "Computed overlay after showPopover()");
    popover_div.hidePopover();
  }, "div.showPopover() should put popover div in top layer");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/popovers/popover-overlay.html"
}
```
