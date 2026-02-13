# html/semantics/interestfor/interestfor-user-select.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-user-select.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://drafts.csswg.org/css-ui/#propdef-user-select">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src='/resources/testdriver-vendor.js'></script>

<button interestfor=foo>ABCDEFGHI<span id=inside>JKLMNOPQ</span>RSTUVWXYZ</button>
<div>Something after the button</div>
<div id=after>Something else</div>

<script>
promise_test(async function() {
  assert_equals(window.getSelection().toString(), "", "Nothing should start out selected");

  await new test_driver.Actions()
    .pointerMove(1, 1, {origin: after})
    .pointerDown({ button: 0 })
    .pointerMove(1, 1, {origin: inside})
    .pointerUp({ button: 0 })
    .send();
  const selection = window.getSelection().toString();
  assert_not_equals(selection, "", "Something should be selected");
  assert_true(selection.includes("Something after"),'The selection should include the outside text');
  assert_false(selection.includes("RSTUVWXYZ"),'The selection should not include the button text');
}, "Buttons with `interestfor` should not be user-selectable");
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
  "source_name": "html/semantics/interestfor/interestfor-user-select.tentative.html"
}
```
