# html/browsers/the-window-object/accessing-other-browsing-contexts/window_length.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/accessing-other-browsing-contexts/window_length.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>window.length</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
var iframe;
var subframe;
var other_window;
test(function() {assert_equals(window.length, 0)}, "No child browsing contexts");
test(function() {
  iframe = document.createElement("iframe");
  assert_equals(window.length, 0)
}, "iframe not inserted into the document");

test(function() {
  document.body.appendChild(iframe);
  assert_equals(window.length, 1)
}, "One iframe inserted into the document");

test(function() {
  subframe = document.createElement("iframe");
  iframe.contentDocument.body.appendChild(subframe);
  assert_equals(window.length, 1);
}, "Child browsing context has a child browsing context");

test(function() {
  try {
    assert_equals(iframe.contentWindow.length, 1);
  } finally {
    subframe.parentNode.removeChild(subframe);
  }
}, "window.length in child frame");

test(function() {
  iframe.parentNode.removeChild(iframe);
  other_window = window.open();
  assert_equals(window.length, 0);
  assert_equals(other_window.length, 0);
}, "Opened window")

test(function() {
  other_window.document.body.appendChild(iframe);
  try {
    assert_equals(other_window.length, 1);
  } finally {
    other_window.close();
  }
}, "Iframe in opened window")

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
  "source_name": "html/browsers/the-window-object/accessing-other-browsing-contexts/window_length.html"
}
```
