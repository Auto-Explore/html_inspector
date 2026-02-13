# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-aboutblank-navigate-immediately.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-aboutblank-navigate-immediately.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>Navigations immediately after appending iframe with src='about:blank'</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
/*
  When an iframe is created with src="about:blank", it will stay on the initial
  empty document. These tests verify the behavior of navigations that happen
  immediately after the iframe is created, which should result in replacement.
*/
"use strict";
const url1 = "/common/blank.html?1";

promise_test(async t => {
  const startingHistoryLength = history.length;
  // Create an iframe with src set to about:blank. Per the HTML Standard, this
  // stays on the "initial about:blank Document" [1], because the "process the
  // iframe attributes" algorithm [2] catches the "about:blank" navigation and
  // fires a special synchronous `load` event at the initial about:blank
  // Document, instead of replacing it with a non-initial, second about:blank
  // Document. This is documented in the note at the end of [3].
  //
  // [1]: https://html.spec.whatwg.org/#is-initial-about:blank.
  // [2]: https://html.spec.whatwg.org/#process-the-iframe-attributes
  // [3]: https://html.spec.whatwg.org/#completely-finish-loading
  const iframe = insertIframeWithAboutBlankSrc(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Trigger a navigation from the initial about:blank Document, to url1 through
  // the iframe's src attribute. Because we're navigating from the initial
  // about:blank Document, the navigation should be a replacement.
  iframe.src = url1;

  // Wait for the latest navigation to finish.
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation from initial " +
    "about:blank Document");
}, "Navigating away from the initial about:blank Document to a different " +
    "one, with src");

promise_test(async t => {
  const startingHistoryLength = history.length;
  const iframe = insertIframeWithAboutBlankSrc(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Trigger a navigation from the initial about:blank Document, to url1 through
  // location.href. Because we're navigating from the initial about:blank
  // Document, the navigation should be a replacement.
  iframe.contentWindow.location.href = url1;
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
}, "Navigating to a different document with location.href");

promise_test(async t => {
  const startingHistoryLength = history.length;
  const iframe = insertIframeWithAboutBlankSrc(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Trigger a navigation from the initial about:blank Document, to url1 through
  // location.assign(). Because we're navigating from the initial about:blank
  // Document, the navigation should be a replacement.
  iframe.contentWindow.location.assign(url1);
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
}, "Navigating to a different document with location.assign");

promise_test(async t => {
  const startingHistoryLength = history.length;
  const iframe = insertIframeWithAboutBlankSrc(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Trigger a navigation from the initial about:blank Document, to url1 through
  // window.open(). Because we're navigating from the initial about:blank
  // Document, the navigation should be a replacement.
  iframe.contentWindow.open(url1, "_self");
  await waitForLoad(t, iframe, url1);
  assert_equals(history.length, startingHistoryLength,
    "history.length must not change after normal navigation on initial empty document");
}, "Navigating to a different document with window.open");

promise_test(async t => {
  const startingHistoryLength = history.length;
  const iframe = insertIframeWithAboutBlankSrc(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Trigger a navigation from the initial about:blank Document, to url1 through
  // clicking an `<a>` element. Because we're navigating from the initial
  // about:blank Document, the navigation should be a replacement.
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
  const iframe = insertIframeWithAboutBlankSrc(t);
  assert_equals(history.length, startingHistoryLength,
    "Inserting iframe with src='about:blank' must not change history.length");

  // Trigger a navigation from the initial about:blank Document, to url1 through
  // a form submission. Because we're navigating from the initial about:blank
  // Document, the navigation should be a replacement.
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
        "byte_end": 297,
        "byte_start": 289,
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
        "byte_end": 6256,
        "byte_start": 297,
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
        "byte_end": 6265,
        "byte_start": 6256,
        "col": 1,
        "line": 130
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/iframe-src-aboutblank-navigate-immediately.html"
}
```
