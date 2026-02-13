# html/semantics/popovers/popover-alignment-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-alignment-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<title>Tests that popover alignment responds to anchor positioning</title>
<link rel="help" href="https://drafts.csswg.org/css-anchor-position-1/#conditional-centering">
<link rel="match" href="popover-alignment-001-ref.html">
<link rel="author" href="mailto:tabatkins@google.com">
<link rel="author" title="Elika J. Etemad" href="http://fantasai.inkedblade.net/contact">

<style>
button {
  border: solid blue 15px;
  margin: 25px;
}
[popover] {
  border: solid orange 10px;
}
</style>

Orange box should be centered on the screen.
<button popovertarget=p1></button>
<div id="p1" popover></div>

<script>
document.querySelector("[popover]").showPopover({ source: document.querySelector("button") });
document.documentElement.removeAttribute("class");
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
  "source_name": "html/semantics/popovers/popover-alignment-001.html"
}
```
