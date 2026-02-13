# html/cross-origin-embedder-policy/none.https.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/none.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<meta name="timeout" content="long">
<title>Cross-Origin-Embedder-Policy header and nested navigable resource without such header</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=/common/utils.js></script> <!-- Use token() to allow running tests in parallel -->
<script src="/common/get-host-info.sub.js"></script>
<div id=log></div>
<script>

const HOST = get_host_info();
const BASE = new URL("resources", location).pathname;

async_test(t => {
  const frame = document.createElement("iframe");
  t.add_cleanup(() => frame.remove());
  frame.onload = t.step_func_done(() => {
    assert_not_equals(frame.contentDocument, null);
  });
  frame.src = "/common/blank.html";
  document.body.append(frame);
  assert_equals(frame.contentDocument.body.localName, "body");
}, `"none" top-level: navigating a frame to "none" should succeed`);

async_test(t => {
  const frame = document.createElement("iframe");
  t.add_cleanup(() => frame.remove());
  const blank = "/common/blank.html";
  let firstNavOk = false;
  frame.onload = t.step_func(() => {
    if (!firstNavOk) {
      assert_not_equals(frame.contentDocument, null);
      firstNavOk = true;
    } else {
      assert_not_equals(frame.contentDocument, null);
      assert_equals(frame.contentWindow.location.pathname, blank);
      t.done();
    }
  });
  frame.src = `resources/navigate-require-corp.sub.html?to=${blank}`;
  document.body.append(frame);
  assert_equals(frame.contentDocument.body.localName, "body");
}, `"none" top-level: navigating a frame from "require-corp" to "none" should succeed`);

async_test(t => {
  let pageLoaded = false;
  // TODO(arthursonzogni): Consider switching toward another message passing
  // API like:
  // /common/dispatcher/dispatcher.js
  const bc = new BroadcastChannel(token());
  let finished = false;
  let doneCheck = _ => {
    if (finished && pageLoaded) {
      t.done();
    }
  }
  bc.onmessage = t.step_func((event) => {
    pageLoaded = true;
    let payload = event.data;
    assert_equals(payload, "loaded");

    doneCheck();
  });

  const bc2 = new BroadcastChannel(token());
  bc2.onmessage = t.step_func((event) => {
    finished = true;
    let payload = event.data;
    assert_equals(payload, "loaded");

    doneCheck();
  });

  const win = window.open(`resources/navigate-require-corp.sub.html?channelName=${bc.name}&to=navigate-none.sub.html?channelName=${bc2.name}`, "_blank", "noopener");
  assert_equals(win, null);
}, `"require-corp" top-level noopener popup: navigating to "none" should succeed`);

async_test(t => {
  const frame = document.createElement("iframe");
  const id = token();
  t.add_cleanup(() => frame.remove());
  window.addEventListener('message', t.step_func((e) => {
    if (e.data === id) {
      // Loaded!
      t.done();
    }
  }));
  frame.src = `${HOST.HTTPS_NOTSAMESITE_ORIGIN}${BASE}/navigate-require-corp-same-site.sub.html?token=${id}`;
  document.body.append(frame);
}, 'CORP: same-site is not checked.');

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 36,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/cross-origin-embedder-policy/none.https.html"
}
```
