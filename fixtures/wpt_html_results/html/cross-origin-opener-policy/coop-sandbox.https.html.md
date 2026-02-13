# html/cross-origin-opener-policy/coop-sandbox.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coop-sandbox.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Sandboxed Cross-Origin-Opener-Policy popup should result in a network error</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/utils.js"></script> <!-- Use token() to allow running tests in parallel -->
<div id=log>
<script>
[
  "allow-popups allow-scripts allow-same-origin",
  "allow-popups allow-scripts",
].forEach(sandboxValue => {
  async_test(t => {
    const frame = document.createElement("iframe");
    const channel = new BroadcastChannel(token());
    channel.onmessage = t.unreached_func("A COOP popup was created from a sandboxed frame");
    t.add_cleanup(() => frame.remove());
    frame.sandbox = sandboxValue;
    frame.srcdoc = `<script>
  const popup = window.open("resources/coop-coep.py?coop=same-origin&coep=&channel=${channel.name}");
  <\/script>`;
    document.body.append(frame);
    addEventListener('load', t.step_func(() => {
      // This uses a timeout to give some time for incorrect implementations to broadcast. A
      // theoretical testdriver.js API for browsing contexts could be used to speed this up.
      t.step_timeout(() => {
        t.done()
      }, 1500);
    }));
  }, `<iframe sandbox="${sandboxValue}"> ${document.title}`);
});

// Verify that the popup does not have sandboxing flags set
async_test(t => {
  const frame = document.createElement("iframe");
  const channel = new BroadcastChannel(token());
  channel.onmessage = t.step_func_done();
  t.add_cleanup(() => frame.remove());
  frame.sandbox = "allow-popups allow-scripts allow-popups-to-escape-sandbox";
  frame.srcdoc = `<script>
window.open("resources/coop-coep.py?coop=same-origin&coep=&channel=${channel.name}");
<\/script>`;
  document.body.append(frame);
}, `<iframe sandbox="allow-popups allow-scripts allow-popups-to-escape-sandbox"> ${document.title}`);

async_test(t => {
  const frame = document.createElement("iframe");
  const channel = new BroadcastChannel(token());
  frame.sandbox = "allow-scripts allow-same-origin";
  frame.name = `iframe-${channel.name}`;
  frame.src = `resources/coop-coep.py?coop=same-origin&coep=&channel=${channel.name}`;
  channel.onmessage = t.step_func( event => {
    const payload = event.data;
    assert_equals(payload.name, frame.name, "name");
    t.done();
  });
  t.add_cleanup(() => frame.remove());
  document.body.append(frame);
}, `Iframe with sandbox and COOP must load.`);
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
  "source_name": "html/cross-origin-opener-policy/coop-sandbox.https.html"
}
```
