# html/browsers/sandboxing/sandbox-window-open-srcdoc.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-window-open-srcdoc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>window.open("about:srcdoc") from a sandboxed iframe</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
function waitForEvent(name, target) {
  return new Promise(resolve => {
    function listener(event) {
      resolve(event);
      }
    target.addEventListener(name, listener, { once: true });
  });
}

// Check what happens when executing window.open("about:srcdoc") from a
// sandboxed iframe. Srcdoc can't be loaded in the main frame. It should
// result in an error page. The error page should be cross-origin with the
// opener.
//
// This test covers an interesting edge case. A main frame should inherit
// sandbox flags. However the document loaded is an internal error page. This
// might trigger some assertions, especially if the implementation wrongly
// applies the sandbox flags of the opener to the internal error page document.
//
// This test is mainly a coverage test. It passes if it doesn't crash.
promise_test(async t => {
  let iframe = document.createElement("iframe");
  iframe.sandbox = "allow-scripts allow-popups allow-same-origin";
  iframe.srcdoc = `
    <script>
      let w = window.open();
      onunload = () => w.close();

      let notify = () => {
        try {
          w.origin; // Will fail after navigating to about:srcdoc.
          parent.postMessage("pending", "*");
        } catch (e) {
          parent.postMessage("done", "*");
        };
      };

      addEventListener("message", notify);
      notify();

      w.location = "about:srcdoc"; // Error page.
    </scr`+`ipt>
  `;

  let msg = waitForEvent("message", window);
  document.body.appendChild(iframe);
  while ( (await msg).data !== "done" ) {
    iframe.contentWindow.postMessage("ping","*");
    msg = waitForEvent("message", window);
  }
  iframe.remove();
}, "window.open('about:srcdoc') from sandboxed srcdoc doesn't crash.");

promise_test(async t => {
  let ifr = document.createElement("iframe");
  ifr.sandbox = "allow-scripts allow-popups";
  ifr.srcdoc = `<script>
    const w = window.open();
    try {
      w.document;
      parent.postMessage("fail", "*")
    } catch (e) {
      parent.postMessage(e.name, "*")
    }
  </scri`+`pt>`;

  const msg = waitForEvent("message", window);
  document.body.appendChild(ifr);
  const data = (await msg).data;
  assert_equals(data, "SecurityError", "");
}, "popup is isolated from an isolated iframe");
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
  "source_name": "html/browsers/sandboxing/sandbox-window-open-srcdoc.html"
}
```
