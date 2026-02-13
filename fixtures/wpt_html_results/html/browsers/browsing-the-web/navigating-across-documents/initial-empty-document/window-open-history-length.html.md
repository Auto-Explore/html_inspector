# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-history-length.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-history-length.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>history.length value on window.open()-ed window</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
/*
  When a new window is opened through window.open() it will contain the initial
  empty document, and the history.length value should be 1.
*/

promise_test(async t => {
  const openedWindow = windowOpenNoURL(t);
  assert_equals(openedWindow.history.length, 1,
    "history.length should start at 1 for newly opened window");
}, "Starting history.length for window.open()");

promise_test(async t => {
  const openedWindow = windowOpenAboutBlank(t);
  assert_equals(openedWindow.history.length, 1,
    "history.length should start at 1 for newly opened window");
}, "Starting history.length for window.open(about:blank)");

promise_test(async t => {
  const openedWindow = windowOpen204(t);
  assert_equals(openedWindow.history.length, 1,
    "history.length should start at 1 for newly opened window");
}, "Starting history.length for window.open(url-with-204-response)");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 275,
        "byte_start": 267,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1153,
        "byte_start": 275,
        "col": 9,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1162,
        "byte_start": 1153,
        "col": 1,
        "line": 31
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-history-length.html"
}
```
