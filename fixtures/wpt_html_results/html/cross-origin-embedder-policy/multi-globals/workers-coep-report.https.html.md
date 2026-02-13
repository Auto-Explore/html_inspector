# html/cross-origin-embedder-policy/multi-globals/workers-coep-report.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/multi-globals/workers-coep-report.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Multiple globals for Worker constructor: COEP reports</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- This is the entry global -->

<iframe src="incumbent/incumbent.html"></iframe>
<button onclick="" id="button">Hello</button>

<script>
async function observeReports(global) {
  const reports = [];
  const observer = new global.ReportingObserver((rs) => {
    for (const r of rs) {
      reports.push(r.toJSON());
    }
  });
  observer.observe();

  // Wait 5000ms for reports to settle.
  await new Promise(r => step_timeout(r, 5000));
  return reports;
}

async_test((t) => {
  onload = t.step_func(() => {
    Promise.all([
      observeReports(window),
      observeReports(frames[0]),
      observeReports(frames[0].frames[0])
    ]).then(t.step_func_done(([entry, incumbent, current]) => {
      assert_equals(entry.length, 0);
      assert_equals(incumbent.length, 0);
      assert_equals(current.length, 1);
      const report = current[0];
      assert_equals(report.type, 'coep');
      assert_equals(report.url, new URL('current/current.html', location.href).href);
      assert_equals(report.body.type, 'worker initialization');
      assert_equals(report.body.blockedURL, new URL('current/worker.js', location.href).href);
      assert_equals(report.body.disposition, 'enforce');
    }));

    frames[0].hello();
  });
  onmessage = t.unreached_func('worker should have been blocked by COEP');
});
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
  "source_name": "html/cross-origin-embedder-policy/multi-globals/workers-coep-report.https.html"
}
```
