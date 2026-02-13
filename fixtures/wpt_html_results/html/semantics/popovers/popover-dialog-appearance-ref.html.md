# html/semantics/popovers/popover-dialog-appearance-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-dialog-appearance-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<title>Dialog-Popover appearance</title>
<link rel="author" href="mailto:masonf@chromium.org">

<p>Both dialogs should have the same shades of background.</p>
<p>The popover should have a completely-transparent ::backdrop.</p>
<dialog popover id=d1>This is a modal dialog</dialog>
<dialog popover id=d2>This is a dialog popover</dialog>

<style>
  dialog {
    left: 50px;
    right: auto;
    bottom: auto;
  }
  #d1 {top:100px;}
  #d2 {top:150px;}
  /* Force backdrop to spec: */
  #d1::backdrop {
    /* https://html.spec.whatwg.org/multipage/rendering.html#flow-content-3 */
    background-color: rgba(0, 0, 0, 0.1);
  }
  #d2::backdrop {
    /* When shown as a popover, backdrop must be transparent */
    background-color: transparent;
  }
</style>

<script>
  document.getElementById('d1').showModal();
  document.getElementById('d2').showPopover();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 386,
        "byte_start": 379,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/popovers/popover-dialog-appearance-ref.html"
}
```
