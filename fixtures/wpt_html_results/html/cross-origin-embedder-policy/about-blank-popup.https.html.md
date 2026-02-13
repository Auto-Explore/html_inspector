# html/cross-origin-embedder-policy/about-blank-popup.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/about-blank-popup.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/script-factory.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script>
  const origins = get_host_info();

  promise_test(async t => {
    const popup = window.open();
    t.add_cleanup(() => { popup.close(); });

    let data_from_popup = () => new Promise(resolve =>
      window.addEventListener("message", (({ data }) => resolve(data))));

    let check_result = (data, text) => {
      assert_equals(data.origin, origin);
      assert_true(data.sameOriginNoCORPSuccess,
                  text + ": Same-origin without CORP did not succeed");
      assert_true(data.crossOriginNoCORPFailure,
                  text + ": Cross-origin without CORP did not fail");
    };

    // Check if COEP is inherited by the popup.
    let script = popup.document.createElement('script');
    script.innerHTML =
      `${createScript(window.origin, origins.HTTPS_REMOTE_ORIGIN, "opener")}`;
    popup.document.body.appendChild(script);
    check_result(await data_from_popup(), "Initial empty document");

    // Navigate the popup away.
    popup.location = origins.HTTPS_REMOTE_ORIGIN +
      "/html/cross-origin-embedder-policy/resources/postmessage-ready.html";
    assert_equals(await new Promise(resolve =>
      window.addEventListener("message", msg => resolve(msg.data))),
      "ready");

    // Navigate the popup to about:blank and wait for it.
    popup.location = "about:blank";
    await t.step_wait(
      condition = () => {
        try {
          return popup.location.href === "about:blank";
        } catch {}
        return false;
      },
      description = "Wait for the popup to navigate.",
      timeout=3000,
      interval=50);

    // Check again if COEP is inherited.
    script = popup.document.createElement('script');
    script.innerHTML =
      `${createScript(window.origin, origins.HTTPS_REMOTE_ORIGIN, "opener")}`;
    popup.document.body.appendChild(script);
    check_result(await data_from_popup(), "Non-initial about:blank document");
  }, `Cross-Origin-Embedder-Policy is inherited by about:blank popup.`);
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
  "source_name": "html/cross-origin-embedder-policy/about-blank-popup.https.html"
}
```
