# html/browsers/browsing-the-web/history-traversal/event-order/after-load-hash-twice.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/event-order/after-load-hash-twice.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Popstate/hashchange/load event ordering</title>

<script>
// Set these up super-early before we hit the network for the test harness, just in case.
window.eventOrder = [];
window.onhashchange = () => window.eventOrder.push("hashchange");
window.onpopstate = () => window.eventOrder.push("popstate");
window.onload = () => window.eventOrder.push("load");
</script>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
async_test(t => {
  assert_array_equals(window.eventOrder, []);

  // 0 timeout is necessary because if we do location.hash assignment before load is finished firing it counts as a replacement.
  window.addEventListener("load", () => t.step_timeout(() => {
    assert_array_equals(window.eventOrder, ["load"]);

    window.addEventListener("hashchange", t.step_func(() => {
      assert_array_equals(window.eventOrder, ["load", "popstate", "popstate", "hashchange"]);

      window.addEventListener("hashchange", t.step_func_done(() => {
        assert_array_equals(window.eventOrder, ["load", "popstate", "popstate", "hashchange", "hashchange"]);
      }));
    }), { once: true });

    location.hash = "#1";
    assert_array_equals(window.eventOrder, ["load", "popstate"]);
    location.hash = "#2";
    assert_array_equals(window.eventOrder, ["load", "popstate", "popstate"]);
  }, 0));
}, "when changing hash, after the load event");
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/event-order/after-load-hash-twice.html"
}
```
