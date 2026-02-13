# html/cross-origin-embedder-policy/require-corp-worker-script-revalidation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/require-corp-worker-script-revalidation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>COEP and dedicated worker</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="resources/worker-support.js"></script>
<body>
<script>

promise_test(async (t) => {
  const worker1 = new Worker("/html/cross-origin-embedder-policy/resources/dedicated-worker-supporting-revalidation.py");
  worker1.onerror = t.unreached_func('Worker.onerror should not be called for first worker');
  worker1.postMessage("foo");
  const result1 = await waitForMessage(worker1);
  assert_equals(result1.data, 'LOADED');

  // Load the worker a second time, which should trigger revalidation of the cached resource.
  const worker2 = new Worker("/html/cross-origin-embedder-policy/resources/dedicated-worker-supporting-revalidation.py");
  worker2.onerror = t.unreached_func('Worker.onerror should not be called worker second worker');
  worker2.postMessage("foo");
  const result2 = await waitForMessage(worker2);
  assert_equals(result2.data, 'LOADED');
}, 'COEP: require-corp with revalidated worker script');
</script>
</body>
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
  "source_name": "html/cross-origin-embedder-policy/require-corp-worker-script-revalidation.html"
}
```
