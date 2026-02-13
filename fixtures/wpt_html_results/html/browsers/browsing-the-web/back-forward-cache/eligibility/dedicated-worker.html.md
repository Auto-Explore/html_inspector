# html/browsers/browsing-the-web/back-forward-cache/eligibility/dedicated-worker.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/back-forward-cache/eligibility/dedicated-worker.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="../resources/helper.sub.js"></script>
<script>
// Check whether the page is BFCached when there are dedicated workers that are
// already loaded.
runBfcacheTest({
  funcBeforeNavigation: async () => {
    globalThis.worker = new Worker('../resources/echo-worker.js');
    // Make sure the worker starts before navigation.
    await WorkerHelper.pingWorker(globalThis.worker);
  },
  funcAfterAssertion: async (pageA) => {
    // Confirm that the worker is still there.
    assert_equals(
      await pageA.execute_script(() => WorkerHelper.pingWorker(globalThis.worker)),
      'PASS',
      'Worker should still work after restored from BFCache');
  }
}, 'Eligibility: dedicated workers');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/back-forward-cache/eligibility/dedicated-worker.html"
}
```
