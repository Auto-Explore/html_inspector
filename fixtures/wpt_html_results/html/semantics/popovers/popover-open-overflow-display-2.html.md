# html/semantics/popovers/popover-open-overflow-display-2.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-open-overflow-display-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/popover-utils.js"></script>
<script>
async function checkStatus(p) {
    p.showPopover();
    await waitForRender();
    assert_true(p.matches(":popover-open"));
    p.hidePopover();
    await waitForRender();
}
</script>

<div id=container style="overflow: hidden; position: absolute;">
    <div popover="auto" id=p1 style="position: absolute; top: 100px;">Absolute popover inside absolute element</div>
</div>
<script>
promise_test(async () => {
    await checkStatus(document.querySelector("#p1"));
}, "Absolute popover inside absolute element");
</script>

<div id=p2 popover="auto" style="overflow: hidden; position: absolute;">
    <div style="position: absolute; top: 100px;">Absolute element inside absolute popover</div>
</div>
<script>
promise_test(async () => {
    await checkStatus(document.querySelector("#p2"));
}, "Absolute element inside absolute popover");
</script>

<div id=container style="overflow: hidden; position: fixed;">
  <div popover="auto" id=p3 style="position: fixed; top: 100px;">Fixed popover inside fixed element</div>
</div>
<script>
promise_test(async () => {
    await checkStatus(document.querySelector("#p3"));
}, "Fixed popover inside fixed element");
</script>

<div id=p4 popover="auto" style="overflow: hidden; position: fixed;">
    <div style="position: fixed; top: 100px;">Fixed element inside fixed popover</div>
</div>
<script>
promise_test(async () => {
    await checkStatus(document.querySelector("#p4"));
}, "Fixed element inside fixed popover");
</script>

<div id=container style="overflow: hidden; position: fixed;">
  <div popover="auto" id=p5 style="position: absolute; top: 100px;">Absolute popover inside fixed element</div>
</div>
<script>
promise_test(async () => {
    await checkStatus(document.querySelector("#p5"));
}, "Absolute popover inside fixed element");
</script>

<div id=p6 popover="auto" style="overflow: hidden; position: absolute;">
  <div style="position: fixed; top: 100px;">Fixed element inside absolute popover</div>
</div>
<script>
promise_test(async () => {
    await checkStatus(document.querySelector("#p6"));
}, "Fixed element inside absolute popover");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “container”.",
      "severity": "Error",
      "span": {
        "byte_end": 1190,
        "byte_start": 1129,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “container”.",
      "severity": "Error",
      "span": {
        "byte_end": 1815,
        "byte_start": 1754,
        "col": 1,
        "line": 53
      }
    },
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
  "source_name": "html/semantics/popovers/popover-open-overflow-display-2.html"
}
```
