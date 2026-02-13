# html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-url.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script type="module" src="import-meta-url.any.js"></script>

<script type="module">
const base = location.href.slice(0, location.href.lastIndexOf('/'));

test(() => {
  assert_equals(import.meta.url, location.href);
}, "import.meta.url in a root inline script");

for (const workerType of ['DedicatedWorker', 'SharedWorker']) {
  promise_test(async t => {
      const worker_request_url =
        new URL(`postmessage-worker.js?${workerType}`, location);
      let w;
      let port;
      if (workerType === 'DedicatedWorker') {
        w = new Worker(worker_request_url.href, {type: 'module'});
        port = w;
      } else {
        w = new SharedWorker(worker_request_url.href, {type: 'module'});
        port = w.port;
        w.port.start();
      }
      w.onerror = t.unreached_func('Worker error');
      const url = await new Promise(resolve => {
        port.onmessage = evt => resolve(evt.data);
      });
      assert_equals(url, worker_request_url.href);
  }, `import.meta.url at top-level module ${workerType}`);
}
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/import-meta/import-meta-url.html"
}
```
