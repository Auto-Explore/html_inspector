# html/webappapis/the-windoworworkerglobalscope-mixin/Worker_Self_Origin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/the-windoworworkerglobalscope-mixin/Worker_Self_Origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test workers self.origin</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
function assertOriginWorker(workerSource, expectedOrigin, testName) {
  async_test(function(t) {
    w = new Worker(workerSource);
    w.onmessage = t.step_func(function(e) {
      assert_equals(e.data, expectedOrigin);
      t.done();
    });
  }, testName + ' Worker');
}

function assertOriginSharedWorker(workerSource, expectedOrigin, testName) {
  async_test(function(t) {
    w = new SharedWorker(workerSource);
    w.port.start();
    w.port.onmessage = t.step_func(function(e) {
      assert_equals(e.data, expectedOrigin);
      t.done();
    });
  }, testName + ' SharedWorker');
}

// Test same-origin workers
assertOriginWorker("./support/WorkerSelfOriginWorker.js", self.origin, "Same Origin");
assertOriginSharedWorker("./support/WorkerSelfOriginSharedWorker.js", self.origin, "Same Origin");

// Test data url workers have opaque origin
assertOriginWorker("data:application/javascript,postMessage(self.origin);", "null", "Data Url");
assertOriginSharedWorker("data:application/javascript,onconnect = function(e) { e.ports[0].postMessage(self.origin); }", "null", "Data Url");

// Test blob url workers
blob = new Blob(["postMessage(self.origin);"]);
blobUrl = URL.createObjectURL(blob);
assertOriginWorker(blobUrl, self.origin, "Blob Url");

blob = new Blob(["onconnect = function(e) { e.ports[0].postMessage(self.origin); }"]);
blobUrl = URL.createObjectURL(blob);
assertOriginSharedWorker(blobUrl, self.origin, "Blob Url");
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
  "source_name": "html/webappapis/the-windoworworkerglobalscope-mixin/Worker_Self_Origin.html"
}
```
