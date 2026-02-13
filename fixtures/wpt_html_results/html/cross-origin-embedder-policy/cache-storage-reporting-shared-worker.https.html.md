# html/cross-origin-embedder-policy/cache-storage-reporting-shared-worker.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/cache-storage-reporting-shared-worker.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
  <title>
    Check COEP report are send for CacheStorage requests in SharedWorker
  </title>
  <meta name="timeout" content="long">
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="/common/get-host-info.sub.js"></script>
  <script src="/common/utils.js"></script>
  <script src="./resources/cache-storage-reporting.js"> </script>
</head>
<script>

promise_test(async (t) => {
  const worker_url = local(encode(worker_path + header_coep));
  const worker = new SharedWorker(worker_url);
  const mc = new MessageChannel();
  worker.port.postMessage({script: eval_script, port: mc.port2}, [mc.port2]);
  const reports = (await new Promise(r => mc.port1.onmessage = r)).data;
  assert_equals(reports.length, 1);
  const report = reports[0];
  assert_equals(report.body.blockedURL, image_url);
  assert_equals(report.body.type, "corp");
  assert_equals(report.body.disposition, "enforce");
  assert_equals(report.body.destination, "");
  assert_equals(report.type, "coep");
  assert_equals(report.url, worker_url);
}, "COEP support on SharedWorker.")

promise_test(async (t) => {
  const worker_url = local(encode(worker_path + header_coep_report_only));
  const worker = new SharedWorker(worker_url);
  const mc = new MessageChannel();
  worker.port.postMessage({script: eval_script, port: mc.port2}, [mc.port2]);
  const reports = (await new Promise(r => mc.port1.onmessage = r)).data;
  assert_equals(reports.length, 1);
  const report = reports[0];
  assert_equals(report.body.blockedURL, image_url);
  assert_equals(report.body.type, "corp");
  assert_equals(report.body.disposition, "reporting");
  assert_equals(report.body.destination, "");
  assert_equals(report.type, "coep");
  assert_equals(report.url, worker_url);
}, "COEP-Report-Only support on SharedWorker.")

</script>
</html>
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
  "source_name": "html/cross-origin-embedder-policy/cache-storage-reporting-shared-worker.https.html"
}
```
