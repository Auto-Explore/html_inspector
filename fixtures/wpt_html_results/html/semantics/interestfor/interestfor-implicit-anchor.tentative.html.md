# html/semantics/interestfor/interestfor-implicit-anchor.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-implicit-anchor.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Interest invokers form an implicit anchor reference</title>
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<button id=button interestfor=popover>Button</button>
<div popover id=popover>popover</div>

<style>
#button {
  position:relative;
  top:100px;
  left:100px;
  interest-delay: 0s;
}
#popover {
  border:1px solid black;
  inset: auto;
  margin:0;
  padding:0;
  position-area: top left;
}
body { margin: 0; }
</style>

<script>
promise_test(async (t) => {
  let interestFired = false;
  popover.addEventListener('interest',() => interestFired = true);

  await hoverOver(button);
  assert_true(popover.matches(':popover-open'));

  // The popover should be anchored to the button.
  assert_equals(popover.offsetLeft + popover.offsetWidth, button.offsetLeft);
  assert_equals(popover.offsetTop + popover.offsetHeight, button.offsetTop);
}, 'Interest invokers form an implicit anchor reference');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 641,
        "byte_start": 634,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/interestfor/interestfor-implicit-anchor.tentative.html"
}
```
