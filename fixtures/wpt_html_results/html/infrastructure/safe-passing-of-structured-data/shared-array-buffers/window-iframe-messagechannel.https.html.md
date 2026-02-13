# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-iframe-messagechannel.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-iframe-messagechannel.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Structured cloning of SharedArrayBuffers into windows using MessageChannel</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#structuredserialize">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/test-incrementer.js"></script>
<script src="/common/get-host-info.sub.js"></script>

<div id="log"></div>

<script>
promise_test(t => {
  return new Promise(resolve => {
    const iframe = document.createElement("iframe");
    window.onmessage = t.step_func((message) => {
      // data will be a MessagePort
      resolve(testSharingViaIncrementerScript(t, message.data, "window", message.data, "iframe"));
    });
    iframe.src = "resources/incrementer-iframe-messagechannel.html";
    document.body.appendChild(iframe);
  });
}, `postMessaging to a same-origin iframe via MessageChannel allows them to see each others' modifications`);

promise_test(t => {
  return new Promise(resolve => {
    const iframe = document.createElement("iframe");
    window.onmessage = t.step_func((message) => {
      // data will be a MessagePort
      message.data.postMessage(new SharedArrayBuffer(10));
      message.data.onmessage = t.step_func(message => {
        assert_equals(message.data, "messageerror event received");
        resolve();
      });
    });
    iframe.src = get_host_info().HTTPS_REMOTE_ORIGIN + new URL("resources/iframe-messagechannel-site-failure.html", window.location).pathname;
    document.body.appendChild(iframe);
  });
}, "postMessaging to a same-site iframe via MessageChannel should fail");

promise_test(t => {
  return new Promise(resolve => {
    const iframe = document.createElement("iframe");
    window.onmessage = t.step_func(message => {
      // data will be a MessagePort
      message.data.postMessage(new SharedArrayBuffer(10));
      message.data.onmessage = t.step_func(message => {
        assert_equals(message.data, "messageerror event received");
        resolve();
      });
    });
    iframe.src = get_host_info().HTTPS_NOTSAMESITE_ORIGIN + new URL("resources/iframe-messagechannel-failure.html", window.location).pathname;
    document.body.append(iframe);
  });
}, "postMessaging to a cross-site iframe via MessageChannel should fail");

promise_test(t => {
  return new Promise(resolve => {
    const iframe = document.createElement("iframe");
    let port = null;
    window.onmessage = t.step_func(message => {
      if (message.data.state === "port1") {
        port = message.data.data;
        port.postMessage(new SharedArrayBuffer(10));
        message.source.postMessage("send port2", "*");
      } else if (message.data.state === "port2") {
        // Note that onmessage calls start()
        message.data.data.onmessage = t.step_func(message => {
          assert_true(message.data instanceof SharedArrayBuffer);
          assert_equals(message.data.byteLength, 10);
          resolve();
        });
        message.data.data.onmessageerror = t.unreached_func();
      }
    });
    iframe.src = get_host_info().HTTPS_NOTSAMESITE_ORIGIN + new URL("resources/iframe-messagechannel-complex.html", window.location).pathname;
    document.body.append(iframe);
  });
}, "postMessaging with a MessageChannel that's been cross-site should succeed");
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-iframe-messagechannel.https.html"
}
```
