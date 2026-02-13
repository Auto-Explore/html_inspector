# html/browsers/browsing-the-web/history-traversal/hashchange-same-fragment-does-not-fire.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/hashchange-same-fragment-does-not-fire.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>hashchange is not fired when setting the same fragment</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
promise_test(async t => {
  t.add_cleanup(() => history.replaceState(null, "", location.pathname + location.search));

  const first = new Promise(resolve => addEventListener("hashchange", resolve, { once: true }));
  location.hash = "#foo";
  await first;

  const secondHashchange = new Promise(resolve => {
    addEventListener("hashchange", () => resolve("fired"), { once: true });
  });
  const timeout = new Promise(resolve => t.step_timeout(() => resolve("timeout"), 100));

  location.hash = "#foo";

  const result = await Promise.race([secondHashchange, timeout]);
  assert_equals(result, "timeout", "hashchange must not fire when fragment does not change");
}, "Setting the same location.hash twice must not fire hashchange the second time");
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/hashchange-same-fragment-does-not-fire.html"
}
```
