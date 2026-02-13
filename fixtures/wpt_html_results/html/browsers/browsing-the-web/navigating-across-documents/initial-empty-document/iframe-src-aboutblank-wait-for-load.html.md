# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-aboutblank-wait-for-load.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-aboutblank-wait-for-load.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>Navigations after iframe with src='about:blank' finished loading</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
  /*
    When an iframe is created with src="about:blank", it will stay on the initial
    empty document. These tests verify the behavior of navigations that happen
    immediately after the load event is fired on the iframe element, which
    should result in replacement.
  */
"use strict";
const url1 = "/common/blank.html?1";

promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to about:blank, and wait for it to finish
  // loading. This would trigger and commit a navigation to a non-initial
  // about:blank document.
  const iframe = await insertIframeWithAboutBlankSrcWaitForLoad(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Trigger a navigation to url1 through the iframe's src attribute.
  // The iframe should still be on the initial empty document, and the
  // navigation should do replacement.
  iframe.src = url1;
  // Wait for the latest navigation to finish.
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
}, "Navigating to a different document with src");

promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to about:blank, and wait for it to finish
  // loading. This would trigger and commit a navigation to a non-initial
  // about:blank document.
  const iframe = await insertIframeWithAboutBlankSrcWaitForLoad(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Navigate away from the initial empty document through setting
  // location.href. The iframe should still be on the initial empty document,
  // and the navigation should do replacement.
  iframe.contentWindow.location.href = url1;
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
 }, "Navigating to a different document with location.href");

promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to about:blank, and wait for it to finish
  // loading. This would trigger and commit a navigation to a non-initial
  // about:blank document.
  const iframe = await insertIframeWithAboutBlankSrcWaitForLoad(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Navigate away from the initial empty document through setting
  // location.href. The iframe should still be on the initial empty document,
  // and the navigation should do replacement.
  iframe.contentWindow.location.href = url1;
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
}, "Navigating to a different document with location.assign");

promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to about:blank, and wait for it to finish
  // loading. This would trigger and commit a navigation to a non-initial
  // about:blank document.
  const iframe = await insertIframeWithAboutBlankSrcWaitForLoad(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Navigate away from the initial empty document through window.open().
  // The iframe should still be on the initial empty document, and the
  // navigation should do replacement.
  iframe.contentWindow.open(url1, "_self");
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
}, "Navigating to a different document with window.open");

promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to about:blank, and wait for it to finish
  // loading. This would trigger and commit a navigation to a non-initial
  // about:blank document.
  const iframe = await insertIframeWithAboutBlankSrcWaitForLoad(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Navigate away from the initial empty document through clicking an <a>
  // element. The iframe should still be on the initial empty document, and the
  // navigation should do replacement.
  const a = iframe.contentDocument.createElement("a");
  a.href = url1;
  iframe.contentDocument.body.appendChild(a);
  a.click();
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
}, "Navigating to a different document with link click");

promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to about:blank which will commit an about:blank document that is not the initial empty document, and wait for it to load.
  const iframe = await insertIframeWithAboutBlankSrcWaitForLoad(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Navigate away from the initial empty document through form submission.
  // The iframe should still be on the initial empty document, and the
  // navigation should do replacement.
  const form = iframe.contentDocument.createElement("form");
  form.action = "/common/blank.html";
  iframe.contentDocument.body.appendChild(form);
  const input = iframe.contentDocument.createElement("input");
  input.type = "hidden";
  input.name = "1";
  form.append(input);
  form.submit();
  await waitForLoad(t, iframe, url1 + "=");
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
}, "Navigating to a different document with form submission");
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
        "byte_end": 292,
        "byte_start": 284,
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
        "byte_end": 6549,
        "byte_start": 292,
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
        "byte_end": 6558,
        "byte_start": 6549,
        "col": 1,
        "line": 134
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-aboutblank-wait-for-load.html"
}
```
