# html/browsers/browsing-the-web/history-traversal/event-order/before-load-replaceState.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/event-order/before-load-replaceState.html",
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

  t.step_timeout(t.step_func_done(() => {
    assert_array_equals(window.eventOrder, ["load"]);
  }), 100);

  history.replaceState({ state: "new state" }, "");
}, "when replacing state, before load");
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/event-order/before-load-replaceState.html"
}
```
