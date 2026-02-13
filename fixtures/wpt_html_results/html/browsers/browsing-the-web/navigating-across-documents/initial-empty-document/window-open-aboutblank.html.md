# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-aboutblank.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-aboutblank.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>Navigations on window.open(about:blank) after waiting for it to load</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
/*
  When a new window is opened through window.open() it will contain the initial
  empty document. If the URL parameter is set to about:blank, it will stay on
  the initial empty document (unlike iframes with src="about:blank", which will
  start a navigation to a non-initial about:blank document).
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
  // Open a window with URL about:blank, which will commit the
  // initial empty document and stay on it.
  const openedWindow = windowOpenAboutBlank(t);

  // Unlike iframe with src="about:blank", window.open("about:blank") won't
  // trigger a navigation to a non-initial about:blank document, so it should
  // stay on the initial empty document. To verify, wait for 100ms before
  // continuing.
  await new Promise((resolve) => t.step_timeout(resolve, 100));

  // Navigate away from the initial empty document through location.href.
  // This should do a replacement.
  openedWindow.location.href = url1;
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 1,
    "history.length should not increase after normal navigation away from initial empty document");
}, "location.href");

promise_test(async t => {
  // Open a window with URL about:blank, which will commit the
  // initial empty document and stay on it.
  const openedWindow = windowOpenAboutBlank(t);

  // Unlike iframe with src="about:blank", window.open("about:blank") won't
  // trigger a navigation to a non-initial about:blank document, so it should
  // stay on the initial empty document. To verify, wait for 100ms before
  // continuing.
  await new Promise((resolve) => t.step_timeout(resolve, 100));

  // Navigate away from the initial empty document through location.assign().
  // This should do a replacement.
  openedWindow.location.assign(url1);
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 1,
    "history.length should not increase after normal navigation away from initial empty document");
}, "location.assign");
/*
promise_test(async t => {
  // Open a window with URL about:blank, which will commit the
  // initial empty document and stay on it.
  const openedWindow = windowOpenAboutBlank(t);

  // Unlike iframe with src="about:blank", window.open("about:blank") won't
  // trigger a navigation to a non-initial about:blank document, so it should
  // stay on the initial empty document. To verify, wait for 100ms before
  // continuing.
  await new Promise((resolve) => t.step_timeout(resolve, 100));

  // Navigate away from the initial empty document through window.open().
  // This should do a replacement.
  openedWindow.open(url1, "_self");
  await waitForMessage(t, "loaded");
  assert_equals(openedWindow.history.length, 1,
    "history.length should not increase after normal navigation away from initial empty document");
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
        "byte_end": 296,
        "byte_start": 288,
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
        "byte_end": 3584,
        "byte_start": 296,
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
        "byte_end": 3593,
        "byte_start": 3584,
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/window-open-aboutblank.html"
}
```
