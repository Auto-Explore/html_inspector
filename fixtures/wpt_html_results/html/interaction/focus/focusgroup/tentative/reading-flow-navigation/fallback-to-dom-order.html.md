# html/interaction/focus/focusgroup/tentative/reading-flow-navigation/fallback-to-dom-order.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/focusgroup/tentative/reading-flow-navigation/fallback-to-dom-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: focusgroup - Falls back to DOM order when no reading flow</title>
<link rel="author" title="Microsoft" href="http://www.microsoft.com/">
<link rel="help" href="https://open-ui.org/components/scoped-focusgroup.explainer/">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="../resources/focusgroup-utils.js"></script>

<div focusgroup="toolbar">
  <span id=item1 tabindex=0>item1</span>
  <span id=item2 tabindex=0>item2</span>
  <span id=item3 tabindex=0>item3</span>
</div>

<script>

  promise_test(async t => {
    // Without reading-flow, should follow DOM order: item1, item2, item3
    const elementsInDOMOrder = [
      document.getElementById("item1"),
      document.getElementById("item2"),
      document.getElementById("item3")
    ];

    await assert_arrow_navigation_bidirectional(elementsInDOMOrder);
  }, "Focusgroup navigation should fall back to DOM order when no reading-flow is specified.");

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
  "source_name": "html/interaction/focus/focusgroup/tentative/reading-flow-navigation/fallback-to-dom-order.html"
}
```
