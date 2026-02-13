# html/semantics/interactive-elements/the-dialog-element/default-color.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/default-color.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Test for dialog element colors</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
:root { background-color: Canvas; color: CanvasText; }
#light { color-scheme: light }
#dark { color-scheme: dark }
</style>
<dialog id="dialog" open>
  This is a dialog
</dialog>
<dialog id="light" open>
  This is a dialog
</dialog>
<dialog id="dark" open>
  This is a dialog
</dialog>
<script>
test(function() {
  let dialog = document.getElementById("dialog");
  let cs = getComputedStyle(dialog);
  let rootCs = getComputedStyle(document.documentElement);
  assert_equals(cs.color, rootCs.color, "Dialog color should match CanvasText");
  assert_equals(cs.backgroundColor, rootCs.backgroundColor, "Dialog background should match Canvas");
}, "<dialog> color and background match default")

test(function() {
  let lightCs = getComputedStyle(document.getElementById("light"));
  let darkCs = getComputedStyle(document.getElementById("dark"));
  assert_not_equals(lightCs.color, darkCs.color, "Dialog color should react to color-scheme");
  assert_not_equals(lightCs.backgroundColor, darkCs.backgroundColor, "Dialog background should react to color-scheme");
}, "<dialog> colors react to dark mode")
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
  "source_name": "html/semantics/interactive-elements/the-dialog-element/default-color.html"
}
```
