# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-pageshow-events-window-open.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-pageshow-events-window-open.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>"load" and "pageshow" events don't fire on window.open() that stays on the initial empty document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
"use strict";

promise_test(async t => {
  const w = window.open();
  return assertNoLoadAndPageshowEvent(t, w)
}, "load event does not fire on window.open()");

promise_test(async t => {
  const w = window.open("");
  return assertNoLoadAndPageshowEvent(t, w)
}, "load event does not fire on window.open('')");

promise_test(async t => {
  const w = window.open("about:blank");
  return assertNoLoadAndPageshowEvent(t, w)
}, "load event does not fire on window.open('about:blank')");

promise_test(async t => {
  const w = window.open("about:blank#foo");
  return assertNoLoadAndPageshowEvent(t, w)
}, "load event does not fire on window.open('about:blank#foo')");

promise_test(async t => {
  const w = window.open("about:blank?foo");
  return assertNoLoadAndPageshowEvent(t, w)
}, "load event does not fire on window.open('about:blank?foo')");
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
        "byte_end": 325,
        "byte_start": 317,
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
        "byte_end": 1173,
        "byte_start": 325,
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
        "byte_end": 1182,
        "byte_start": 1173,
        "col": 1,
        "line": 35
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-pageshow-events-window-open.html"
}
```
