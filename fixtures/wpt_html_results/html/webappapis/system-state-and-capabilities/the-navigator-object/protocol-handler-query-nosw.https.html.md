# html/webappapis/system-state-and-capabilities/the-navigator-object/protocol-handler-query-nosw.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/system-state-and-capabilities/the-navigator-object/protocol-handler-query-nosw.https.html",
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
<title>registerProtocolHandler() and a handler with %s in the query (does not use a service worker)</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src=/service-workers/service-worker/resources/test-helpers.sub.js></script>
<script>
'use strict';
promise_setup(async () => {
  await test_driver.set_rph_registration_mode("autoAccept");
  await test_driver.bless('handler registration');
  register();
});
// Configure expectations for individual test
window.type = "query";
window.noSW = true;
</script>
<script src=resources/handler-tools.js></script>
<script>
runTest();
</script>
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
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/system-state-and-capabilities/the-navigator-object/protocol-handler-query-nosw.https.html"
}
```
