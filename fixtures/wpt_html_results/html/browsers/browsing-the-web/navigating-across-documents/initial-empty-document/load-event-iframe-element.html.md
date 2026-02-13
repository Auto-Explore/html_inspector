# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-event-iframe-element.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-event-iframe-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>"load" event fires on the iframe element when loading the initial empty document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
"use strict";

promise_test(async t => {
  const iframe = document.createElement("iframe");
  let loadCount = 0;
  iframe.addEventListener("load", () => {
    loadCount++;
  });
  document.body.appendChild(iframe);
  assert_equals(loadCount, 1);
  assert_equals(iframe.contentDocument.location.href, "about:blank");
}, "load event fires synchronously on <iframe> element created with no src");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "";
  let loadCount = 0;
  iframe.addEventListener("load", () => {
    loadCount++;
  });
  document.body.appendChild(iframe);
  assert_equals(loadCount, 1);
  assert_equals(iframe.contentDocument.location.href, "about:blank");
}, "load event fires synchronously on <iframe> element created with src=''");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank";
  let loadCount = 0;
  iframe.addEventListener("load", () => {
    loadCount++;
  });
  document.body.appendChild(iframe);
  assert_equals(loadCount, 1);
  assert_equals(iframe.contentDocument.location.href, "about:blank");
}, "load event fires synchronously on <iframe> element created with src='about:blank'");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank#foo";
  let loadCount = 0;
  iframe.addEventListener("load", () => {
    loadCount++;
  });
  document.body.appendChild(iframe);
  assert_equals(loadCount, 1);
  assert_equals(iframe.contentDocument.location.href, "about:blank#foo");
}, "load event fires synchronously on <iframe> element created with src='about:blank#foo'");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank?foo";
  let loadCount = 0;
  iframe.addEventListener("load", () => {
    loadCount++;
  });
  document.body.appendChild(iframe);
  assert_equals(loadCount, 1);
  assert_equals(iframe.contentDocument.location.href, "about:blank?foo");
}, "load event fires synchronously on <iframe> element created with src='about:blank?foo'");
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
        "byte_end": 308,
        "byte_start": 300,
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
        "byte_end": 2389,
        "byte_start": 308,
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
        "byte_end": 2398,
        "byte_start": 2389,
        "col": 1,
        "line": 69
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-event-iframe-element.html"
}
```
