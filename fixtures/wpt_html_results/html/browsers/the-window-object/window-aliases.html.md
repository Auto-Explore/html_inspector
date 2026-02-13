# html/browsers/the-window-object/window-aliases.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/window-aliases.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Aliases of the window object</title>
<link rel="author" title="Ms2ger" href="mailto:Ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-window">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-frames">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-self">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var global = this;

test(function() {
  assert_equals(window, global);
  assert_equals(window.window, global);
}, "window should be the global object");

test(function() {
  assert_equals(frames, global);
  assert_equals(window.frames, global);
}, "frames should be the global object");

test(function() {
  assert_equals(self, global);
  assert_equals(window.self, global);
}, "self should be the global object");
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
  "source_name": "html/browsers/the-window-object/window-aliases.html"
}
```
