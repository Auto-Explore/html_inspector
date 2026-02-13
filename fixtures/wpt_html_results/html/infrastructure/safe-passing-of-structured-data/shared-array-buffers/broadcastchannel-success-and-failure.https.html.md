# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/broadcastchannel-success-and-failure.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/broadcastchannel-success-and-failure.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>SharedArrayBuffer cannot cross agent clusters, BroadcastChannel edition</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
async_test(t => {
  const channel = new BroadcastChannel("anne was here"),
        dw = new Worker("resources/broadcastchannel-worker.js"),
        sw = new SharedWorker("resources/broadcastchannel-sharedworker.js");
  let startCounter = 0,
      dwStatus = "unknown",
      swStatus = "unknown";

  channel.onmessage = t.step_func(({ data }) => {
    if(data === "hi") {
      startCounter++;
      if(startCounter === 2) {
        const sab = new SharedArrayBuffer();
        channel.postMessage(sab);
      } else if(startCounter > 2) {
        assert_unreached();
      }
    } else if(data === "dw-success") {
      dwStatus = "success";
    } else if(data === "sw-success") {
      swStatus = "success";
    } else {
      assert_unreached();
    }
    if(dwStatus === "success" && swStatus === "success") {
      t.done();
    }
  });
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/broadcastchannel-success-and-failure.https.html"
}
```
