# html/semantics/popovers/popover-types.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-types.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="container">
  <div popover>Popover</div>
  <div popover=manual>Async</div>
  <div popover=manual>Async</div>
</div>
<script>
  const auto = container.querySelector('[popover=""]');
  const manual = container.querySelectorAll('[popover=manual]')[0];
  const manual2 = container.querySelectorAll('[popover=manual]')[1];
  function assert_state_1(autoOpen,manualOpen,manual2Open) {
    assert_equals(auto.matches(':popover-open'),autoOpen,'auto open state is incorrect');
    assert_equals(manual.matches(':popover-open'),manualOpen,'manual open state is incorrect');
    assert_equals(manual2.matches(':popover-open'),manual2Open,'manual2 open state is incorrect');
  }
  test(() => {
    assert_state_1(false,false,false);
    auto.showPopover();
    assert_state_1(true,false,false);
    manual.showPopover();
    assert_state_1(true,true,false);
    manual2.showPopover();
    assert_state_1(true,true,true);
    auto.hidePopover();
    assert_state_1(false,true,true);
    manual.hidePopover();
    assert_state_1(false,false,true);
    manual2.hidePopover();
    assert_state_1(false,false,false);
  }, 'manuals do not close popovers');
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
  "source_name": "html/semantics/popovers/popover-types.html"
}
```
