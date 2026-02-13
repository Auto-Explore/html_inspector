# html/semantics/scripting-1/the-script-element/json-module/json-module-service-worker-test.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/json-module/json-module-service-worker-test.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
    promise_test(async (test) => {
        const reg = await navigator.serviceWorker.register('./serviceworker.js', { type: 'module' });
        test.add_cleanup(() => reg.unregister());
        assert_not_equals(reg.installing, undefined);
    }, "Javascript importing JSON Module should load within the context of a service worker");

    promise_test(test => {
        return promise_rejects_dom(test, "SecurityError",
            navigator.serviceWorker.register('./module.json', { type: 'module' }),
            "Attempting to load JSON as a service worker should fail");
    }, "Trying to register a service worker with a top-level JSON Module should fail");

    promise_test(async (test) => {
        const reg = await navigator.serviceWorker.register('./serviceworker-dynamic-import.js', { type: 'module' });
        test.add_cleanup(() => reg.unregister());
        assert_not_equals(reg.installing, undefined);
        reg.installing.postMessage("PING");
        const msgEvent = await new Promise(resolve => {
            navigator.serviceWorker.onmessage = resolve;
        });
        assert_equals(msgEvent.data, "FAILED");
    }, "JSON Module dynamic import should not load within the context of a service worker");
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
  "source_name": "html/semantics/scripting-1/the-script-element/json-module/json-module-service-worker-test.https.html"
}
```
