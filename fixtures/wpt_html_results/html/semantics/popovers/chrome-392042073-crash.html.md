# html/semantics/popovers/chrome-392042073-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/chrome-392042073-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Chrome hidePopover() display:none transition crash</title>
<link rel="help" href="https://crbug.com/392042073">
<style>
  #target {
    transition-property: overlay, display;
    transition-duration: 3600s;
    transition-behavior: allow-discrete;
  }
</style>
<p>PASS if no crash</p>
<div style="position-area:top;position:absolute"></div>
<button id="btn" popovertarget="target">First</button>
<div id="target" popover="hint"></div>
<script>
  btn.click();
  target.hidePopover();
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
  "source_name": "html/semantics/popovers/chrome-392042073-crash.html"
}
```
