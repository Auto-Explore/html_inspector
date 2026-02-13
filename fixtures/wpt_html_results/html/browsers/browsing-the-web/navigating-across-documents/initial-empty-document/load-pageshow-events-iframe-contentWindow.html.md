# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-pageshow-events-iframe-contentWindow.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-pageshow-events-iframe-contentWindow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>"load" & "pageshow" events do not fire on contentWindow of iframe that stays on the initial empty document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
"use strict";

promise_test(async t => {
  const iframe = document.createElement("iframe");
  document.body.appendChild(iframe);
  return assertNoLoadAndPageshowEvent(t, iframe.contentWindow);
}, "load & pageshow event do not fire on contentWindow of <iframe> element created with no src");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "";
  document.body.appendChild(iframe);
  return assertNoLoadAndPageshowEvent(t, iframe.contentWindow);
}, "load & pageshow events do not fire on contentWindow of <iframe> element created with src=''");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank";
  document.body.appendChild(iframe);
  return assertNoLoadAndPageshowEvent(t, iframe.contentWindow);
}, "load & pageshow events do not fire on contentWindow of <iframe> element created with src='about:blank'");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank#foo";
  document.body.appendChild(iframe);
  return assertNoLoadAndPageshowEvent(t, iframe.contentWindow);
}, "load & pageshow events do not fire on contentWindow of <iframe> element created with src='about:blank#foo'");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank?foo";
  document.body.appendChild(iframe);
  return assertNoLoadAndPageshowEvent(t, iframe.contentWindow);
}, "load & pageshow events do not fire on contentWindow of <iframe> element created with src='about:blank?foo'");
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
        "byte_end": 334,
        "byte_start": 326,
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
        "byte_end": 1896,
        "byte_start": 334,
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
        "byte_end": 1905,
        "byte_start": 1896,
        "col": 1,
        "line": 44
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/load-pageshow-events-iframe-contentWindow.html"
}
```
