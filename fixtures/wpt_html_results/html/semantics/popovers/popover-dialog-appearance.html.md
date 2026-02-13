# html/semantics/popovers/popover-dialog-appearance.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-dialog-appearance.html",
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
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel="match" href="popover-dialog-appearance-ref.html">

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
        "byte_end": 603,
        "byte_start": 596,
        "col": 1,
        "line": 14
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
  "source_name": "html/semantics/popovers/popover-dialog-appearance.html"
}
```
