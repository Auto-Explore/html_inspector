# html/document-isolation-policy/isolate-and-require-corp.tentative.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/document-isolation-policy/isolate-and-require-corp.tentative.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="timeout" content="long">
<title>Document-Isolation-Policy: isolate-and-require-corp header and subresources</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script> <!-- Use token() to allow running tests in parallel -->
<div id=log></div>
<script>
const HOST = get_host_info();
const BASE = new URL("resources", location).pathname;

promise_test(async t => {
  const response = await fetch(get_host_info().HTTPS_REMOTE_ORIGIN+"/html/document-isolation-policy/resources/nothing-cross-origin-corp.js", {mode: "no-cors"});
  assert_equals(response.type, "opaque");
}, `"isolate-and-require-corp" top-level: fetch() to CORP: cross-origin response should succeed`);

promise_test(async t => {
  return promise_rejects_js(t, TypeError, fetch(get_host_info().HTTPS_REMOTE_ORIGIN+"/html/document-isolation-policy/resources/nothing-no-corp.js", {mode: "no-cors"}));
}, `"isolate-and-require-corp" top-level: fetch() to cross-origin response without CORP should fail`);

</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/document-isolation-policy/isolate-and-require-corp.tentative.https.html"
}
```
