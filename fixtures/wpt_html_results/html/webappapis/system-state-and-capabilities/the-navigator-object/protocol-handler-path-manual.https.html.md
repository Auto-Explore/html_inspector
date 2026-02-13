# html/webappapis/system-state-and-capabilities/the-navigator-object/protocol-handler-path-manual.https.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/protocol-handler-path-manual.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<!-- Use a non-UTF-8 encoding to see how the handler URL is parsed -->
<meta charset=windows-1254>
<meta name=timeout content=long>
<title>registerProtocolHandler() and a handler with %s in the path</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/service-workers/service-worker/resources/test-helpers.sub.js></script>
<script>
// Configure expectations for individual test
window.type = "path";
window.noSW = false;
</script>
<script src=resources/handler-tools.js></script>
<ol>
 <li><p>First, register the handler: <button onclick='register()'>Register</button>.
 <li><p>Then, run the test: <button onclick='runTest()'>Run</button>.
 <li><p>Or, run the test with U+0000 NULL: <button onclick='runTest({ includeNull: true })'>Run NULL</button>.
</ol>
<div id=log></div>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1254” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 114,
        "byte_start": 87,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.parse.p.end_tag_implied_open_elements",
      "message": "End tag “p” implied, but there were open elements.",
      "severity": "Error",
      "span": {
        "byte_end": 664,
        "byte_start": 661,
        "col": 6,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.parse.p.end_tag_implied_open_elements",
      "message": "End tag “p” implied, but there were open elements.",
      "severity": "Error",
      "span": {
        "byte_end": 734,
        "byte_start": 731,
        "col": 6,
        "line": 18
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
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/protocol-handler-path-manual.https.html"
}
```
