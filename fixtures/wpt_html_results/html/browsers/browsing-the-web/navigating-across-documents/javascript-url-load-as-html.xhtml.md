# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-load-as-html.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-load-as-html.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" encoding="windows-1250"?>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="../resources/helpers.js"></script>
  <meta charset="windows-1250"/>
  <title>javascript: URL navigation to a string must create a HTML document using the correct properties</title>
</head>
<body>
  <!--
    This document is XHTML and windows-1250 so that we can test the resulting javascript: URL document is not.
    The same for the window we open.
  -->
  <script><![CDATA[
  promise_test(async (t) => {
    const w = await openWindow("resources/xhtml-and-non-utf-8.xhtml", t);

    w.location.href = `javascript:'a string<script>
      opener.postMessage({
        compatMode: document.compatMode,
        contentType: document.contentType,
        characterSet: document.characterSet,
        doctypeIsNull: document.doctype === null
      }, "*");
    <` + `/script>'`;

    const results = await waitForMessage(w);

    assert_equals(results.compatMode, "BackCompat");
    assert_equals(results.contentType, "text/html");
    assert_equals(results.characterSet, "UTF-8");
    assert_equals(results.doctypeIsNull, true);
  });
  ]]></script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1250” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 289,
        "byte_start": 259,
        "col": 3,
        "line": 7
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-load-as-html.xhtml"
}
```
