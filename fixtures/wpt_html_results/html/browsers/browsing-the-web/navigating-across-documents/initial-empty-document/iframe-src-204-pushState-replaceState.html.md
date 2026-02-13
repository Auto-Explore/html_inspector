# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-204-pushState-replaceState.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-204-pushState-replaceState.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>pushState/replaceState on iframe with src set to URL that doesn't load a document (HTTP 204)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
/*
  When an iframe is created it will contain the initial empty document. If the
  src is set to a URL that doesn't load a new document (e.g. it results in a
  HTTP 204 response), it will stay on the initial empty document. If
  history.pushState() or history.replaceState() is called on it, it should
  still stay on the initial empty document.
  These tests verifies the behavior of navigations that happen on the initial
  empty document in that situation. They should all be converted to do a
  replacement.
*/
"use strict";
const crossDocumentURL = "/common/blank.html?2";

promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to URL that doesn't load a new document, so
  // it will stay on the initial empty document.
  const iframe = insertIframeWith204Src(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src set to URL that doesn't load a new document must not change history.length");

  // Do a history.pushState() to about:blank#foo.
  let pushURL = "about:blank#foo";
  iframe.contentWindow.history.pushState({}, "title", pushURL);
  assert_equals(iframe.contentWindow.location.href, pushURL);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after history.pushState() on the initial empty document");

  // Navigate away from the initial empty document. This should do replacement.
  iframe.src = crossDocumentURL;
  await waitForLoad(t, iframe, crossDocumentURL);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation from initial empty document");
}, "history.pushState");


promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to URL that doesn't load a new document, so
  // it will stay on the initial empty document.
  const iframe = insertIframeWith204Src(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src set to URL that doesn't load a new document must not change history.length");

  // Do a history.replaceState() to about:blank#foo.
  let replaceURL = "about:blank#foo";
  iframe.contentWindow.history.replaceState({}, "title", replaceURL);
  assert_equals(iframe.contentWindow.location.href, replaceURL);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after history.replaceState() on the initial empty document");

  // Navigate away from the initial empty document. This should do replacement.
  iframe.src = crossDocumentURL;
  await waitForLoad(t, iframe, crossDocumentURL);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation from initial empty document");
}, "history.replaceState");
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
        "byte_end": 320,
        "byte_start": 312,
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
        "byte_end": 3138,
        "byte_start": 320,
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
        "byte_end": 3147,
        "byte_start": 3138,
        "col": 1,
        "line": 66
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-204-pushState-replaceState.html"
}
```
