# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-204.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-204.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>Navigations on window.open() with URL that doesn't load a new document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
/*
  When a new window is opened through window.open() it will contain the initial
  empty document. If the URL parameter is set to the URL that doesn't load a
  new document (e.g. it results in a HTTP 204 response), it will stay on the
  initial empty document.
  These tests verify the behavior of navigations that happen on the initial
  empty document in that situation. They should all be converted to do a
  replacement.
*/
"use strict";
const url1 = "resources/code-injector.html?1&pipe=sub(none)&code=" +
              encodeURIComponent(postMessageToOpenerOnLoad);
const url2 = "resources/code-injector.html?2&pipe=sub(none)&code=" +
              encodeURIComponent(postMessageToOpenerOnLoad);

promise_test(async t => {
  // Open a new window with a URL that doesn't load a new document, so it will stay in the initial empty document.
  const openedWindow = windowOpen204(t);

  // Navigate away from the initial empty document through setting
  // location.href. This should do a replacement.
  openedWindow.location.href = url1;
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 1,
    "history.length should not increase after normal navigation away from initial empty document");

  // Navigate again using the same method, but this time it shouldn't do a
  // replacement since it's no longer on the initial empty document.
  openedWindow.location.href = url2;
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 2,
    "history.length should increase after normal navigation away from non-initial empty document");
}, "location.href");

promise_test(async t => {
  // Open a new window with a URL that doesn't load a new document, so it will stay in the initial empty document.
  const openedWindow = windowOpen204(t);

  // Navigate away from the initial empty document through location.assign().
  // This should do a replacement.
  openedWindow.location.assign(url1);
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 1,
    "history.length should not increase after normal navigation away from initial empty document");

  // Navigate again using the same method, but this time it shouldn't do a
  // replacement since it's no longer on the initial empty document.
  openedWindow.location.assign(url2);
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 2,
    "history.length should increase after normal navigation away from non-initial empty document");
}, "location.assign");
/*
promise_test(async t => {
  // Open a new window with a URL that doesn't load a new document, so it will stay in the initial empty document.
  const openedWindow = windowOpen204(t);

  // Navigate away from the initial empty document through setting
  // window.open(). This should do a replacement.
  openedWindow.open(url1, "_self");
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 1,
    "history.length should increase after normal navigation away from non-initial empty document");

  // Navigate again using the same method, but this time it shouldn't do a
  // replacement since it's no longer on the initial empty document.
  openedWindow.open(url2, "_self");
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 2,
    "history.length should increase after normal navigation away from non-initial empty document");
}, "window.open");
*/
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
        "byte_end": 298,
        "byte_start": 290,
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
        "byte_end": 3737,
        "byte_start": 298,
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
        "byte_end": 3746,
        "byte_start": 3737,
        "col": 1,
        "line": 81
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-204.html"
}
```
