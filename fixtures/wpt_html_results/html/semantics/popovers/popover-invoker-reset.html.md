# html/semantics/popovers/popover-invoker-reset.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-invoker-reset.html",
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
<link rel=help href="https://github.com/whatwg/html/issues/9152">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/popover-utils.js"></script>

<div id=p1 popover>Popover 1
  <button popovertarget=p2>Button</button>
</div>
<div id=p2 popover>Popover 2</div>

<script>
  test((t) => {
    p1.showPopover();
    assert_true(p1.matches(':popover-open'));
    const invoker = p1.querySelector('button');
    p2.addEventListener('beforetoggle',(e) => {
      assert_equals(e.newState,'open');
      e.preventDefault();
    },{once:true});
    invoker.click(); // Will be cancelled
    assert_false(p2.matches(':popover-open'));
    assert_true(p1.matches(':popover-open'));
    p2.showPopover();
    assert_true(p2.matches(':popover-open'));
    assert_false(p1.matches(':popover-open'),'invoker was not used to show p2, so p1 should close');
  },'Invoker gets reset appropriately');
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
  "source_name": "html/semantics/popovers/popover-invoker-reset.html"
}
```
