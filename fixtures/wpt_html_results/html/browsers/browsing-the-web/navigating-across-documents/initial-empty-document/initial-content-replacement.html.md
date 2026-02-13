# html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/initial-content-replacement.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/initial-content-replacement.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>
  Content synchronously added to iframe/opened window's document after creation
  won't get replaced asynchronously when staying on the initial empty document
</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<body></body>
<script>
"use strict";

// Asserts the document on |w| stays the same after waiting 100ms.
function assertDocumentStaysTheSame(t, w) {
  const initialDocument = w.document;
  initialDocument.body.innerHTML = "foo";
  return new Promise((resolve) => {
    t.step_timeout(() => {
      assert_equals(w.document.body.innerHTML, "foo");
      assert_equals(w.document, initialDocument);
      resolve();
    }, 100);
  });
}

promise_test(async t => {
  const iframe = document.createElement("iframe");
  document.body.appendChild(iframe);
  await assertDocumentStaysTheSame(t, iframe.contentWindow);
}, "Content synchronously added to <iframe> with no src won't get replaced");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "";
  document.body.appendChild(iframe);
  await assertDocumentStaysTheSame(t, iframe.contentWindow);
}, "Content synchronously added to <iframe> with src='' won't get replaced");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank";
  document.body.appendChild(iframe);
  await assertDocumentStaysTheSame(t, iframe.contentWindow);
}, "Content synchronously added to <iframe> with src='about:blank' won't get replaced");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank#foo";
  document.body.appendChild(iframe);
  await assertDocumentStaysTheSame(t, iframe.contentWindow);
}, "Content synchronously added to <iframe> with src='about:blank#foo' won't get replaced");

promise_test(async t => {
  const iframe = document.createElement("iframe");
  iframe.src = "about:blank?foo";
  document.body.appendChild(iframe);
  await assertDocumentStaysTheSame(t, iframe.contentWindow);
}, "Content synchronously added to <iframe> with src='about:blank?foo' won't get replaced");

promise_test(async t => {
  const w = window.open();
  await assertDocumentStaysTheSame(t, w);
}, "Content synchronously added to window.open()-ed document won't get replaced");

promise_test(async t => {
  const w = window.open("");
  await assertDocumentStaysTheSame(t, w);
}, "Content synchronously added to window.open('')-ed document won't get replaced");

promise_test(async t => {
  const w = window.open("about:blank");
  await assertDocumentStaysTheSame(t, w);
}, "Content synchronously added to window.open('about:blank')-ed document won't get replaced");

promise_test(async t => {
  const w = window.open("about:blank#foo");
  await assertDocumentStaysTheSame(t, w);
}, "Content synchronously added to window.open('about:blank#foo')-ed document won't get replaced");

promise_test(async t => {
  const w = window.open("about:blank?foo");
  await assertDocumentStaysTheSame(t, w);
}, "Content synchronously added to window.open('about:blank?foo')-ed document won't get replaced");

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
        "byte_end": 388,
        "byte_start": 380,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 3223,
        "byte_start": 388,
        "col": 9,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 3232,
        "byte_start": 3223,
        "col": 1,
        "line": 86
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/initial-empty-document/initial-content-replacement.html"
}
```
